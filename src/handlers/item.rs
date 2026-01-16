use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, Set};
use serde_json::json;

use crate::db::DatabaseConnection;
use crate::entities::item::{ActiveModel, CreateItem, Entity, UpdateItem};
use crate::utils::error::AppError;

pub async fn get_items(
    State(db): State<DatabaseConnection>,
) -> Result<impl IntoResponse, AppError> {
    let items = Entity::find()
        .all(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch items: {}", e)))?;

    Ok((
        StatusCode::OK,
        axum::Json(json!({ "items": items })),
    ))
}

pub async fn get_item(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let item = Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch item: {}", e)))?;

    match item {
        Some(i) => Ok((StatusCode::OK, axum::Json(json!(i)))),
        None => Err(AppError::not_found(format!(
            "Item with id {} not found",
            id
        ))),
    }
}

pub async fn create_item(
    State(db): State<DatabaseConnection>,
    axum::Json(payload): axum::Json<CreateItem>,
) -> Result<impl IntoResponse, AppError> {
    let mut active_model = <ActiveModel as sea_orm::ActiveModelTrait>::default();
    active_model.sku = Set(payload.sku);
    active_model.name = Set(payload.name);
    active_model.unit_of_measure = Set(payload.unit_of_measure);
    active_model.barcode = Set(payload.barcode);
    active_model.is_serialized = Set(payload.is_serialized.unwrap_or(false));

    let item = active_model
        .insert(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to create item: {}", e)))?;

    Ok((StatusCode::CREATED, axum::Json(json!(item))))
}

pub async fn update_item(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
    axum::Json(payload): axum::Json<UpdateItem>,
) -> Result<impl IntoResponse, AppError> {
    let item = Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch item: {}", e)))?
        .ok_or_else(|| AppError::not_found(format!("Item with id {} not found", id)))?;

    let mut active_model: ActiveModel = item.into();

    if let Some(sku) = payload.sku {
        active_model.sku = Set(sku);
    }
    if let Some(name) = payload.name {
        active_model.name = Set(name);
    }
    if let Some(unit_of_measure) = payload.unit_of_measure {
        active_model.unit_of_measure = Set(unit_of_measure);
    }
    if payload.barcode.is_some() {
        active_model.barcode = Set(payload.barcode);
    }
    if let Some(is_serialized) = payload.is_serialized {
        active_model.is_serialized = Set(is_serialized);
    }

    let updated = active_model
        .update(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to update item: {}", e)))?;

    Ok((StatusCode::OK, axum::Json(json!(updated))))
}

pub async fn delete_item(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let item = Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch item: {}", e)))?
        .ok_or_else(|| AppError::not_found(format!("Item with id {} not found", id)))?;

    item.delete(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to delete item: {}", e)))?;

    Ok(StatusCode::NO_CONTENT)
}
