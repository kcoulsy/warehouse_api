use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sea_orm::{ActiveModelTrait, Set};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::db::DatabaseConnection;
use crate::entities::warehouse::{ActiveModel, CreateWarehouse};
use crate::utils::error::AppError;
use crate::utils::validation::validate_timezone;

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateWarehouseRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: String,

    #[validate(length(
        min = 1,
        max = 500,
        message = "Address must be between 1 and 500 characters"
    ))]
    pub address: String,

    #[validate(custom(function = "validate_timezone"))]
    pub timezone: String,

    pub is_active: Option<bool>,
}

impl CreateWarehouseRequest {
    pub fn trim_fields(mut self) -> Self {
        self.name = self.name.trim().to_string();
        self.address = self.address.trim().to_string();
        self.timezone = self.timezone.trim().to_string();
        self
    }
}

pub async fn create_warehouse(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateWarehouseRequest>,
) -> Result<impl IntoResponse, AppError> {
    let request = payload.trim_fields();

    request
        .validate()
        .map_err(|e| AppError::validation(AppError::collect_validation_errors(&e)))?;

    let create_dto = CreateWarehouse {
        name: request.name,
        address: request.address,
        timezone: request.timezone,
        is_active: request.is_active,
    };

    let mut active_model = <ActiveModel as sea_orm::ActiveModelTrait>::default();
    active_model.name = Set(create_dto.name);
    active_model.address = Set(create_dto.address);
    active_model.timezone = Set(create_dto.timezone);
    active_model.is_active = Set(create_dto.is_active.unwrap_or(true));

    let warehouse = active_model
        .insert(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to create warehouse: {}", e)))?;

    Ok((StatusCode::CREATED, Json(json!(warehouse))))
}
