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
use crate::entities::warehouse::{ActiveModel, Entity};
use crate::utils::error::AppError;
use crate::utils::validation::validate_timezone;

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateWarehouseRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: Option<String>,

    #[validate(length(
        min = 1,
        max = 500,
        message = "Address must be between 1 and 500 characters"
    ))]
    pub address: Option<String>,

    #[validate(custom(function = "validate_timezone"))]
    pub timezone: Option<String>,

    pub is_active: Option<bool>,
}

impl UpdateWarehouseRequest {
    pub fn trim_fields(mut self) -> Self {
        if let Some(ref mut name) = self.name {
            *name = name.trim().to_string();
        }
        if let Some(ref mut address) = self.address {
            *address = address.trim().to_string();
        }
        if let Some(ref mut timezone) = self.timezone {
            *timezone = timezone.trim().to_string();
        }
        self
    }
}

pub async fn update_warehouse(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateWarehouseRequest>,
) -> Result<impl IntoResponse, AppError> {
    if id <= 0 {
        return Err(AppError::bad_request(
            "Warehouse ID must be a positive integer",
        ));
    }

    let request = payload.trim_fields();

    request
        .validate()
        .map_err(|e| AppError::validation(AppError::collect_validation_errors(&e)))?;

    let warehouse = Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch warehouse: {}", e)))?
        .ok_or_else(|| AppError::not_found(format!("Warehouse with id {} not found", id)))?;

    let mut active_model: ActiveModel = warehouse.into();

    if let Some(name) = request.name {
        active_model.name = Set(name);
    }
    if let Some(address) = request.address {
        active_model.address = Set(address);
    }
    if let Some(timezone) = request.timezone {
        active_model.timezone = Set(timezone);
    }
    if let Some(is_active) = request.is_active {
        active_model.is_active = Set(is_active);
    }

    let updated = active_model
        .update(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to update warehouse: {}", e)))?;

    Ok((StatusCode::OK, Json(json!(updated))))
}
