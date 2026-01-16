use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

use crate::db::DatabaseConnection;
use crate::services::transfer;
use crate::utils::error::AppError;

pub async fn complete_transfer(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    if id <= 0 {
        return Err(AppError::bad_request(
            "Transfer ID must be a positive integer",
        ));
    }

    let result = transfer::complete_transfer(&db, id).await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "transfer_id": result.transfer.id,
            "from_location_id": result.transfer.from_location_id,
            "to_location_id": result.transfer.to_location_id,
            "status": result.transfer.status,
            "lines": result.lines.iter().map(|line| json!({
                "id": line.id,
                "item_id": line.item_id,
                "quantity": line.quantity
            })).collect::<Vec<_>>(),
            "ledger_entries": result.ledger_entries.iter().map(|(source_id, dest_id)| json!({
                "source_ledger_id": source_id,
                "destination_ledger_id": dest_id
            })).collect::<Vec<_>>(),
            "updated_at": result.transfer.updated_at
        })),
    ))
}
