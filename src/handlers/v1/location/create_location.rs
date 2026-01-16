use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sea_orm::{ActiveModelTrait, Set};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::db::DatabaseConnection;
use crate::entities::location::{ActiveModel, CreateLocation};
use crate::utils::error::AppError;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateLocationRequest {
    #[validate(range(min = 1, message = "Warehouse ID must be a positive integer"))]
    pub warehouse_id: i32,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Code must be between 1 and 100 characters"
    ))]
    pub code: String,

    #[validate(length(
        min = 1,
        max = 50,
        message = "Aisle must be between 1 and 50 characters"
    ))]
    pub aisle: String,

    #[validate(length(min = 1, max = 50, message = "Bin must be between 1 and 50 characters"))]
    pub bin: String,

    #[validate(length(
        min = 1,
        max = 50,
        message = "Shelf must be between 1 and 50 characters"
    ))]
    pub shelf: String,

    pub is_pickable: bool,
    pub is_bulk: bool,
}

impl CreateLocationRequest {
    pub fn trim_fields(mut self) -> Self {
        self.code = self.code.trim().to_string();
        self.aisle = self.aisle.trim().to_string();
        self.bin = self.bin.trim().to_string();
        self.shelf = self.shelf.trim().to_string();
        self
    }
}

pub async fn create_location(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateLocationRequest>,
) -> Result<impl IntoResponse, AppError> {
    let request = payload.trim_fields();

    request
        .validate()
        .map_err(|e| AppError::validation(AppError::collect_validation_errors(&e)))?;

    let create_dto = CreateLocation {
        warehouse_id: request.warehouse_id,
        code: request.code,
        aisle: request.aisle,
        bin: request.bin,
        shelf: request.shelf,
        is_pickable: request.is_pickable,
        is_bulk: request.is_bulk,
    };

    let mut active_model = <ActiveModel as sea_orm::ActiveModelTrait>::default();
    active_model.warehouse_id = Set(create_dto.warehouse_id);
    active_model.code = Set(create_dto.code);
    active_model.aisle = Set(create_dto.aisle);
    active_model.bin = Set(create_dto.bin);
    active_model.shelf = Set(create_dto.shelf);
    active_model.is_pickable = Set(create_dto.is_pickable);
    active_model.is_bulk = Set(create_dto.is_bulk);

    let location = active_model
        .insert(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to create location: {}", e)))?;

    Ok((StatusCode::CREATED, Json(json!(location))))
}
