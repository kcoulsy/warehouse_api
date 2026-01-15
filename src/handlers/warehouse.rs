use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, Set};
use serde_json::json;

use crate::db::DatabaseConnection;
use crate::entities::warehouse::{ActiveModel, CreateWarehouse, Entity, UpdateWarehouse};
use crate::error::AppError;

pub async fn get_warehouses(
    State(db): State<DatabaseConnection>,
) -> Result<impl IntoResponse, AppError> {
    let warehouses = Entity::find()
        .all(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch warehouses: {}", e)))?;

    Ok((
        StatusCode::OK,
        axum::Json(json!({
            "warehouses": warehouses
        })),
    ))
}

pub async fn get_warehouse(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let warehouse = Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch warehouse: {}", e)))?;

    match warehouse {
        Some(w) => Ok((StatusCode::OK, axum::Json(json!(w)))),
        None => Err(AppError::not_found(format!("Warehouse with id {} not found", id))),
    }
}

pub async fn create_warehouse(
    State(db): State<DatabaseConnection>,
    axum::Json(payload): axum::Json<CreateWarehouse>,
) -> Result<impl IntoResponse, AppError> {
    let mut active_model = <ActiveModel as sea_orm::ActiveModelTrait>::default();
    active_model.name = Set(payload.name);
    active_model.address = Set(payload.address);
    active_model.timezone = Set(payload.timezone);
    active_model.is_active = Set(payload.is_active.unwrap_or(true));

    let warehouse = active_model
        .insert(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to create warehouse: {}", e)))?;

    Ok((
        StatusCode::CREATED,
        axum::Json(json!(warehouse)),
    ))
}

pub async fn update_warehouse(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
    axum::Json(payload): axum::Json<UpdateWarehouse>,
) -> Result<impl IntoResponse, AppError> {
    let warehouse = Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch warehouse: {}", e)))?
        .ok_or_else(|| AppError::not_found(format!("Warehouse with id {} not found", id)))?;

    let mut active_model: ActiveModel = warehouse.into();

    if let Some(name) = payload.name {
        active_model.name = Set(name);
    }
    if let Some(address) = payload.address {
        active_model.address = Set(address);
    }
    if let Some(timezone) = payload.timezone {
        active_model.timezone = Set(timezone);
    }
    if let Some(is_active) = payload.is_active {
        active_model.is_active = Set(is_active);
    }

    let updated = active_model
        .update(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to update warehouse: {}", e)))?;

    Ok((
        StatusCode::OK,
        axum::Json(json!(updated)),
    ))
}

pub async fn delete_warehouse(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let warehouse = Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch warehouse: {}", e)))?
        .ok_or_else(|| AppError::not_found(format!("Warehouse with id {} not found", id)))?;

    warehouse
        .delete(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to delete warehouse: {}", e)))?;

    Ok(StatusCode::NO_CONTENT)
}
