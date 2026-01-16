use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{EntityTrait, ModelTrait};

use crate::db::DatabaseConnection;
use crate::entities::location::Entity;
use crate::utils::error::AppError;

pub async fn delete_location(
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
        .map_err(|e| AppError::internal(format!("Failed to fetch location: {}", e)))?
        .ok_or_else(|| AppError::not_found(format!("Location with id {} not found", id)))?;

    location
        .delete(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to delete location: {}", e)))?;

    Ok(StatusCode::NO_CONTENT)
}
