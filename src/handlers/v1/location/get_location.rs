use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::EntityTrait;
use serde_json::json;

use crate::db::DatabaseConnection;
use crate::entities::location::Entity;
use crate::utils::error::AppError;

pub async fn get_location(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    if id <= 0 {
        return Err(AppError::bad_request(
            "Location ID must be a positive integer",
        ));
    }

    let location = Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch location: {}", e)))?;

    match location {
        Some(l) => Ok((StatusCode::OK, Json(json!(l)))),
        None => Err(AppError::not_found(format!(
            "Location with id {} not found",
            id
        ))),
    }
}
