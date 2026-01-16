use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, Set};
use serde_json::json;

use crate::db::DatabaseConnection;
use crate::entities::location::{ActiveModel, CreateLocation, Entity, UpdateLocation};
use crate::error::AppError;

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
