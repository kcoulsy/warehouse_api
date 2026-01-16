use axum::{
    Json,
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};
use csv::ReaderBuilder;
use serde_json::json;

use crate::db::DatabaseConnection;
use crate::services::receipt::{BulkReceiptRow, process_bulk_receipt};
use crate::utils::error::AppError;

pub async fn bulk_receipt(
    State(db): State<DatabaseConnection>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let mut csv_data: Option<Vec<u8>> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::bad_request(format!("Failed to read multipart field: {}", e)))?
    {
        let name = field.name().unwrap_or("").to_string();
        let file_name = field.file_name().map(|s| s.to_string());

        // Accept any field that:
        // 1. Has a file_name (indicates file upload)
        // 2. Has name "file" or "csv"
        // 3. Or any field if we haven't found one yet (fallback)
        let is_file_field = file_name.is_some()
            || name == "file"
            || name == "csv"
            || (csv_data.is_none() && !name.is_empty());

        if is_file_field {
            let data = field
                .bytes()
                .await
                .map_err(|e| AppError::bad_request(format!("Failed to read file data: {}", e)))?;

            // Only use this data if it looks like CSV (contains commas or newlines)
            // This helps avoid using field names as data
            let data_vec = data.to_vec();
            if !data_vec.is_empty()
                && (data_vec.contains(&b',')
                    || data_vec.contains(&b'\n')
                    || data_vec.contains(&b'\r'))
            {
                // Prefer explicitly named fields
                if name == "file" || name == "csv" {
                    csv_data = Some(data_vec);
                    break;
                } else if csv_data.is_none() {
                    csv_data = Some(data_vec);
                }
            }
        }
    }

    let csv_bytes = csv_data.ok_or_else(|| {
        AppError::bad_request(
            "No CSV file found in multipart form data. Expected field name 'file' or 'csv'",
        )
    })?;

    // Remove BOM if present (UTF-8 BOM is EF BB BF)
    let csv_bytes_clean = if csv_bytes.len() >= 3
        && csv_bytes[0] == 0xEF
        && csv_bytes[1] == 0xBB
        && csv_bytes[2] == 0xBF
    {
        &csv_bytes[3..]
    } else {
        csv_bytes.as_slice()
    };

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .flexible(true)
        .from_reader(csv_bytes_clean);

    let mut rows = Vec::new();
    let mut row_errors = Vec::new();

    let headers = reader
        .headers()
        .map_err(|e| AppError::bad_request(format!("Failed to read CSV headers: {}", e)))?;

    let header_map: std::collections::HashMap<String, usize> = headers
        .iter()
        .enumerate()
        .map(|(i, h)| (h.trim().to_lowercase(), i))
        .collect();

    let required_columns = ["sku", "location_code", "quantity"];
    for col in required_columns {
        if !header_map.contains_key(col) {
            let found_headers: Vec<String> = header_map.keys().cloned().collect();
            return Err(AppError::bad_request(format!(
                "Missing required column: {}. Found columns: {:?}",
                col, found_headers
            )));
        }
    }

    for (row_index, result) in reader.records().enumerate() {
        let record = match result {
            Ok(r) => r,
            Err(e) => {
                row_errors.push(format!("Row {}: CSV parse error: {}", row_index + 2, e));
                continue;
            }
        };

        let get_field = |col: &str| -> Option<String> {
            header_map
                .get(col)
                .and_then(|&idx| record.get(idx))
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
        };

        let sku = match get_field("sku") {
            Some(s) => s,
            None => {
                row_errors.push(format!(
                    "Row {}: Missing required field 'sku'",
                    row_index + 2
                ));
                continue;
            }
        };

        let location_code = match get_field("location_code") {
            Some(s) => s,
            None => {
                row_errors.push(format!(
                    "Row {}: Missing required field 'location_code'",
                    row_index + 2
                ));
                continue;
            }
        };

        let quantity = match get_field("quantity") {
            Some(q) => match q.parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    row_errors.push(format!("Row {}: Invalid quantity '{}'", row_index + 2, q));
                    continue;
                }
            },
            None => {
                row_errors.push(format!(
                    "Row {}: Missing required field 'quantity'",
                    row_index + 2
                ));
                continue;
            }
        };

        let name = get_field("name");
        let unit_of_measure = get_field("unit_of_measure");
        let barcode = get_field("barcode");
        let is_serialized = get_field("is_serialized").and_then(|s| s.parse::<bool>().ok());
        let warehouse_id = get_field("warehouse_id").and_then(|s| s.parse::<i32>().ok());
        let aisle = get_field("aisle");
        let bin = get_field("bin");
        let shelf = get_field("shelf");
        let is_pickable = get_field("is_pickable").and_then(|s| s.parse::<bool>().ok());
        let is_bulk = get_field("is_bulk").and_then(|s| s.parse::<bool>().ok());

        rows.push(BulkReceiptRow {
            sku,
            location_code,
            quantity,
            name,
            unit_of_measure,
            barcode,
            is_serialized,
            warehouse_id,
            aisle,
            bin,
            shelf,
            is_pickable,
            is_bulk,
        });
    }

    if !row_errors.is_empty() {
        return Ok((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "CSV parsing errors",
                "errors": row_errors
            })),
        ));
    }

    if rows.is_empty() {
        return Err(AppError::bad_request("CSV file contains no valid rows"));
    }

    let result = process_bulk_receipt(&db, rows).await?;

    let errors: Vec<serde_json::Value> = result
        .errors
        .iter()
        .map(|e| {
            json!({
                "row": e.row,
                "error": e.error
            })
        })
        .collect();

    let status = if result.errors.is_empty() {
        StatusCode::CREATED
    } else {
        StatusCode::PARTIAL_CONTENT
    };

    Ok((
        status,
        Json(json!({
            "receipt_id": result.receipt_id,
            "total_rows": result.total_rows,
            "successful_rows": result.successful_rows,
            "errors": errors
        })),
    ))
}
