use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::EntityTrait;
use serde_json::json;

use crate::db::DatabaseConnection;
use crate::entities::warehouse::Entity;
use crate::utils::error::AppError;

pub async fn get_warehouse(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    if id <= 0 {
        return Err(AppError::bad_request(
            "Warehouse ID must be a positive integer",
        ));
    }

    let warehouse = Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch warehouse: {}", e)))?;

    match warehouse {
        Some(w) => Ok((StatusCode::OK, Json(json!(w)))),
        None => Err(AppError::not_found(format!(
            "Warehouse with id {} not found",
            id
        ))),
    }
}
