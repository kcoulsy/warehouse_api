use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sea_orm::EntityTrait;
use serde_json::json;

use crate::db::DatabaseConnection;
use crate::entities::item::Entity;
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
        Json(json!({ "items": items })),
    ))
}
