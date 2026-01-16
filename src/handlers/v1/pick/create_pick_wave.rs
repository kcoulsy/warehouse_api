use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::db::DatabaseConnection;
use crate::services::pick;
use crate::utils::error::AppError;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct PickItemRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "SKU must be between 1 and 100 characters"
    ))]
    pub sku: String,

    #[validate(range(min = 1, message = "Quantity must be a positive integer"))]
    pub quantity: i32,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Location code must be between 1 and 100 characters"
    ))]
    pub location_code: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreatePickWaveRequest {
    #[validate(length(min = 1, message = "At least one item is required"))]
    pub items: Vec<PickItemRequest>,
}

impl CreatePickWaveRequest {
    pub fn trim_fields(mut self) -> Self {
        for item in &mut self.items {
            item.sku = item.sku.trim().to_string();
            item.location_code = item.location_code.trim().to_string();
        }
        self
    }
}

pub async fn create_pick_wave(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreatePickWaveRequest>,
) -> Result<impl IntoResponse, AppError> {
    let request = payload.trim_fields();

    request
        .validate()
        .map_err(|e| AppError::validation(AppError::collect_validation_errors(&e)))?;

    let pick_items: Vec<pick::PickItem> = request
        .items
        .iter()
        .map(|item| pick::PickItem {
            sku: item.sku.clone(),
            quantity: item.quantity,
            location_code: item.location_code.clone(),
        })
        .collect();

    let result = pick::create_pick_wave(&db, pick_items).await?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "pick_wave_id": result.wave.id,
            "status": result.wave.status,
            "lines": result.lines.iter().map(|line| json!({
                "id": line.id,
                "item_id": line.item_id,
                "location_id": line.location_id,
                "quantity": line.quantity,
                "status": line.status
            })).collect::<Vec<_>>(),
            "created_at": result.wave.created_at
        })),
    ))
}
