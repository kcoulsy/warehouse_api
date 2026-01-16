use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sea_orm::EntityTrait;
use serde_json::json;

use crate::db::DatabaseConnection;
use crate::entities::location::Entity;
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
        Json(json!({ "locations": locations })),
    ))
}
