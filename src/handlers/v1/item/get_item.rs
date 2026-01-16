use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::EntityTrait;
use serde_json::json;

use crate::db::DatabaseConnection;
use crate::entities::item::Entity;
use crate::utils::error::AppError;

pub async fn get_item(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    if id <= 0 {
        return Err(AppError::bad_request(
            "Item ID must be a positive integer",
        ));
    }

    let item = Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch item: {}", e)))?;

    match item {
        Some(i) => Ok((StatusCode::OK, Json(json!(i)))),
        None => Err(AppError::not_found(format!(
            "Item with id {} not found",
            id
        ))),
    }
}
