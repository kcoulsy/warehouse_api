use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::db::DatabaseConnection;
use crate::services::receipt;
use crate::utils::error::AppError;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateReceiptRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "SKU must be between 1 and 100 characters"
    ))]
    pub sku: String,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Location code must be between 1 and 100 characters"
    ))]
    pub location_code: String,

    #[validate(range(min = 1, message = "Quantity must be a positive integer"))]
    pub quantity: i32,

    #[validate(length(
        max = 36,
        message = "Receipt ID must be at most 36 characters (UUID format)"
    ))]
    pub receipt_id: Option<String>,
}

impl CreateReceiptRequest {
    pub fn trim_fields(mut self) -> Self {
        self.sku = self.sku.trim().to_string();
        self.location_code = self.location_code.trim().to_string();
        if let Some(ref mut receipt_id) = self.receipt_id {
            *receipt_id = receipt_id.trim().to_string();
        }
        self
    }
}

pub async fn create_receipt(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateReceiptRequest>,
) -> Result<impl IntoResponse, AppError> {
    let request = payload.trim_fields();

    request
        .validate()
        .map_err(|e| AppError::validation(AppError::collect_validation_errors(&e)))?;

    let receipt_id = request
        .receipt_id
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    let item = receipt::find_item_by_sku(&db, &request.sku)
        .await?
        .ok_or_else(|| AppError::not_found(format!("Item with SKU '{}' not found", request.sku)))?;

    let location = receipt::find_location_by_code(&db, &request.location_code)
        .await?
        .ok_or_else(|| {
            AppError::not_found(format!(
                "Location with code '{}' not found",
                request.location_code
            ))
        })?;

    let ledger_entry_id = receipt::create_receipt_ledger_entry(
        &db,
        item.id,
        location.id,
        request.quantity,
        &receipt_id,
    )
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "receipt_id": receipt_id,
            "item_id": item.id,
            "location_id": location.id,
            "quantity": request.quantity,
            "ledger_entry_id": ledger_entry_id
        })),
    ))
}
