use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::db::DatabaseConnection;
use crate::entities::item::{ActiveModel, Entity};
use crate::utils::error::AppError;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateItemRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "SKU must be between 1 and 100 characters"
    ))]
    pub sku: Option<String>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: Option<String>,

    #[validate(length(
        min = 1,
        max = 50,
        message = "Unit of measure must be between 1 and 50 characters"
    ))]
    pub unit_of_measure: Option<String>,

    #[validate(length(
        max = 100,
        message = "Barcode must be at most 100 characters"
    ))]
    pub barcode: Option<String>,

    pub is_serialized: Option<bool>,
}

impl UpdateItemRequest {
    pub fn trim_fields(mut self) -> Self {
        if let Some(ref mut sku) = self.sku {
            *sku = sku.trim().to_string();
        }
        if let Some(ref mut name) = self.name {
            *name = name.trim().to_string();
        }
        if let Some(ref mut unit_of_measure) = self.unit_of_measure {
            *unit_of_measure = unit_of_measure.trim().to_string();
        }
        if let Some(ref mut barcode) = self.barcode {
            *barcode = barcode.trim().to_string();
        }
        self
    }
}

pub async fn update_item(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateItemRequest>,
) -> Result<impl IntoResponse, AppError> {
    if id <= 0 {
        return Err(AppError::bad_request(
            "Item ID must be a positive integer",
        ));
    }

    let request = payload.trim_fields();

    request
        .validate()
        .map_err(|e| AppError::validation(AppError::collect_validation_errors(&e)))?;

    let item = Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch item: {}", e)))?
        .ok_or_else(|| AppError::not_found(format!("Item with id {} not found", id)))?;

    let mut active_model: ActiveModel = item.into();

    if let Some(sku) = request.sku {
        active_model.sku = Set(sku);
    }
    if let Some(name) = request.name {
        active_model.name = Set(name);
    }
    if let Some(unit_of_measure) = request.unit_of_measure {
        active_model.unit_of_measure = Set(unit_of_measure);
    }
    if request.barcode.is_some() {
        active_model.barcode = Set(request.barcode);
    }
    if let Some(is_serialized) = request.is_serialized {
        active_model.is_serialized = Set(is_serialized);
    }

    let updated = active_model
        .update(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to update item: {}", e)))?;

    Ok((StatusCode::OK, Json(json!(updated))))
}
