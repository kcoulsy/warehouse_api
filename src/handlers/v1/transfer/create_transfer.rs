use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::db::DatabaseConnection;
use crate::services::receipt;
use crate::services::transfer;
use crate::utils::error::AppError;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct TransferItemRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "SKU must be between 1 and 100 characters"
    ))]
    pub sku: String,

    #[validate(range(min = 1, message = "Quantity must be a positive integer"))]
    pub quantity: i32,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateTransferRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "From location code must be between 1 and 100 characters"
    ))]
    pub from_location_code: String,

    #[validate(length(
        min = 1,
        max = 100,
        message = "To location code must be between 1 and 100 characters"
    ))]
    pub to_location_code: String,

    #[validate(length(min = 1, message = "At least one item is required"))]
    pub items: Vec<TransferItemRequest>,
}

impl CreateTransferRequest {
    pub fn trim_fields(mut self) -> Self {
        self.from_location_code = self.from_location_code.trim().to_string();
        self.to_location_code = self.to_location_code.trim().to_string();
        for item in &mut self.items {
            item.sku = item.sku.trim().to_string();
        }
        self
    }
}

pub async fn create_transfer(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateTransferRequest>,
) -> Result<impl IntoResponse, AppError> {
    let request = payload.trim_fields();

    request
        .validate()
        .map_err(|e| AppError::validation(AppError::collect_validation_errors(&e)))?;

    let from_location = receipt::find_location_by_code(&db, &request.from_location_code)
        .await?
        .ok_or_else(|| {
            AppError::not_found(format!(
                "Source location with code '{}' not found",
                request.from_location_code
            ))
        })?;

    let to_location = receipt::find_location_by_code(&db, &request.to_location_code)
        .await?
        .ok_or_else(|| {
            AppError::not_found(format!(
                "Destination location with code '{}' not found",
                request.to_location_code
            ))
        })?;

    let transfer_items: Vec<transfer::TransferItem> = request
        .items
        .iter()
        .map(|item| transfer::TransferItem {
            sku: item.sku.clone(),
            quantity: item.quantity,
        })
        .collect();

    let result =
        transfer::create_transfer(&db, from_location.id, to_location.id, transfer_items).await?;

    Ok((
        StatusCode::CREATED,
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
            "created_at": result.transfer.created_at
        })),
    ))
}
