use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sea_orm::{ActiveModelTrait, Set};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::db::DatabaseConnection;
use crate::entities::item::{ActiveModel, CreateItem};
use crate::utils::error::AppError;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateItemRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "SKU must be between 1 and 100 characters"
    ))]
    pub sku: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: String,

    #[validate(length(
        min = 1,
        max = 50,
        message = "Unit of measure must be between 1 and 50 characters"
    ))]
    pub unit_of_measure: String,

    #[validate(length(max = 100, message = "Barcode must be at most 100 characters"))]
    pub barcode: Option<String>,

    pub is_serialized: Option<bool>,
}

impl CreateItemRequest {
    pub fn trim_fields(mut self) -> Self {
        self.sku = self.sku.trim().to_string();
        self.name = self.name.trim().to_string();
        self.unit_of_measure = self.unit_of_measure.trim().to_string();
        if let Some(ref mut barcode) = self.barcode {
            *barcode = barcode.trim().to_string();
        }
        self
    }
}

pub async fn create_item(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateItemRequest>,
) -> Result<impl IntoResponse, AppError> {
    let request = payload.trim_fields();

    request
        .validate()
        .map_err(|e| AppError::validation(AppError::collect_validation_errors(&e)))?;

    let create_dto = CreateItem {
        sku: request.sku,
        name: request.name,
        unit_of_measure: request.unit_of_measure,
        barcode: request.barcode,
        is_serialized: request.is_serialized,
    };

    let mut active_model = <ActiveModel as sea_orm::ActiveModelTrait>::default();
    active_model.sku = Set(create_dto.sku);
    active_model.name = Set(create_dto.name);
    active_model.unit_of_measure = Set(create_dto.unit_of_measure);
    active_model.barcode = Set(create_dto.barcode);
    active_model.is_serialized = Set(create_dto.is_serialized.unwrap_or(false));

    let item = active_model
        .insert(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to create item: {}", e)))?;

    Ok((StatusCode::CREATED, Json(json!(item))))
}
