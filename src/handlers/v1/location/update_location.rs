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
use crate::entities::location::{ActiveModel, Entity};
use crate::utils::error::AppError;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateLocationRequest {
    #[validate(range(min = 1, message = "Warehouse ID must be a positive integer"))]
    pub warehouse_id: Option<i32>,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Code must be between 1 and 100 characters"
    ))]
    pub code: Option<String>,

    #[validate(length(
        min = 1,
        max = 50,
        message = "Aisle must be between 1 and 50 characters"
    ))]
    pub aisle: Option<String>,

    #[validate(length(min = 1, max = 50, message = "Bin must be between 1 and 50 characters"))]
    pub bin: Option<String>,

    #[validate(length(
        min = 1,
        max = 50,
        message = "Shelf must be between 1 and 50 characters"
    ))]
    pub shelf: Option<String>,

    pub is_pickable: Option<bool>,
    pub is_bulk: Option<bool>,
}

impl UpdateLocationRequest {
    pub fn trim_fields(mut self) -> Self {
        if let Some(ref mut code) = self.code {
            *code = code.trim().to_string();
        }
        if let Some(ref mut aisle) = self.aisle {
            *aisle = aisle.trim().to_string();
        }
        if let Some(ref mut bin) = self.bin {
            *bin = bin.trim().to_string();
        }
        if let Some(ref mut shelf) = self.shelf {
            *shelf = shelf.trim().to_string();
        }
        self
    }
}

pub async fn update_location(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateLocationRequest>,
) -> Result<impl IntoResponse, AppError> {
    if id <= 0 {
        return Err(AppError::bad_request(
            "Location ID must be a positive integer",
        ));
    }

    let request = payload.trim_fields();

    request
        .validate()
        .map_err(|e| AppError::validation(AppError::collect_validation_errors(&e)))?;

    let location = Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch location: {}", e)))?
        .ok_or_else(|| AppError::not_found(format!("Location with id {} not found", id)))?;

    let mut active_model: ActiveModel = location.into();

    if let Some(warehouse_id) = request.warehouse_id {
        active_model.warehouse_id = Set(warehouse_id);
    }
    if let Some(code) = request.code {
        active_model.code = Set(code);
    }
    if let Some(aisle) = request.aisle {
        active_model.aisle = Set(aisle);
    }
    if let Some(bin) = request.bin {
        active_model.bin = Set(bin);
    }
    if let Some(shelf) = request.shelf {
        active_model.shelf = Set(shelf);
    }
    if let Some(is_pickable) = request.is_pickable {
        active_model.is_pickable = Set(is_pickable);
    }
    if let Some(is_bulk) = request.is_bulk {
        active_model.is_bulk = Set(is_bulk);
    }

    let updated = active_model
        .update(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to update location: {}", e)))?;

    Ok((StatusCode::OK, Json(json!(updated))))
}
