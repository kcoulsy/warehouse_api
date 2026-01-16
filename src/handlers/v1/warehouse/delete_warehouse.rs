use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{EntityTrait, ModelTrait};

use crate::db::DatabaseConnection;
use crate::entities::warehouse::Entity;
use crate::utils::error::AppError;

pub async fn delete_warehouse(
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
        .map_err(|e| AppError::internal(format!("Failed to fetch warehouse: {}", e)))?
        .ok_or_else(|| AppError::not_found(format!("Warehouse with id {} not found", id)))?;

    warehouse
        .delete(&db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to delete warehouse: {}", e)))?;

    Ok(StatusCode::NO_CONTENT)
}
