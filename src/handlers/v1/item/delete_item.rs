use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{EntityTrait, ModelTrait};

use crate::db::DatabaseConnection;
use crate::entities::item::Entity;
use crate::utils::error::AppError;

pub async fn delete_item(
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
        .map_err(|e| AppError::internal(format!("Failed to fetch item: {}", e)))?
        .ok_or_else(|| AppError::not_found(format!("Item with id {} not found", id)))?;

    item.delete(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to delete item: {}", e)))?;

    Ok(StatusCode::NO_CONTENT)
}
