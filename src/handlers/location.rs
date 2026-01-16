use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, Set};
use serde_json::json;

use crate::db::DatabaseConnection;
use crate::entities::location::{ActiveModel, CreateLocation, Entity, UpdateLocation};
use crate::utils::error::AppError;

pub async fn get_locations(
    State(db): State<DatabaseConnection>,
) -> Result<impl IntoResponse, AppError> {
    let locations = Entity::find()
        .all(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch locations: {}", e)))?;

    Ok((
        StatusCode::OK,
        axum::Json(json!({ "locations": locations })),
    ))
}

pub async fn get_location(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let location = Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch location: {}", e)))?;

    match location {
        Some(l) => Ok((StatusCode::OK, axum::Json(json!(l)))),
        None => Err(AppError::not_found(format!(
            "Location with id {} not found",
            id
        ))),
    }
}

pub async fn create_location(
    State(db): State<DatabaseConnection>,
    axum::Json(payload): axum::Json<CreateLocation>,
) -> Result<impl IntoResponse, AppError> {
    let mut active_model = <ActiveModel as sea_orm::ActiveModelTrait>::default();
    active_model.warehouse_id = Set(payload.warehouse_id);
    active_model.code = Set(payload.code);
    active_model.aisle = Set(payload.aisle);
    active_model.bin = Set(payload.bin);
    active_model.shelf = Set(payload.shelf);
    active_model.is_pickable = Set(payload.is_pickable);
    active_model.is_bulk = Set(payload.is_bulk);

    let location = active_model
        .insert(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to create location: {}", e)))?;

    Ok((StatusCode::CREATED, axum::Json(json!(location))))
}

pub async fn update_location(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
    axum::Json(payload): axum::Json<UpdateLocation>,
) -> Result<impl IntoResponse, AppError> {
    let location = Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch location: {}", e)))?
        .ok_or_else(|| AppError::not_found(format!("Location with id {} not found", id)))?;

    let mut active_model: ActiveModel = location.into();

    if let Some(warehouse_id) = payload.warehouse_id {
        active_model.warehouse_id = Set(warehouse_id);
    }
    if let Some(code) = payload.code {
        active_model.code = Set(code);
    }
    if let Some(aisle) = payload.aisle {
        active_model.aisle = Set(aisle);
    }
    if let Some(bin) = payload.bin {
        active_model.bin = Set(bin);
    }
    if let Some(shelf) = payload.shelf {
        active_model.shelf = Set(shelf);
    }
    if let Some(is_pickable) = payload.is_pickable {
        active_model.is_pickable = Set(is_pickable);
    }
    if let Some(is_bulk) = payload.is_bulk {
        active_model.is_bulk = Set(is_bulk);
    }

    let updated = active_model
        .update(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to update location: {}", e)))?;

    Ok((StatusCode::OK, axum::Json(json!(updated))))
}

pub async fn delete_location(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let location = Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch location: {}", e)))?
        .ok_or_else(|| AppError::not_found(format!("Location with id {} not found", id)))?;

    location
        .delete(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to delete location: {}", e)))?;

    Ok(StatusCode::NO_CONTENT)
}
