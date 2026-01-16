use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

use crate::db::DatabaseConnection;
use crate::services::pick;
use crate::utils::error::AppError;

pub async fn allocate_pick_wave(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    if id <= 0 {
        return Err(AppError::bad_request(
            "Pick wave ID must be a positive integer",
        ));
    }

    let result = pick::allocate_pick_wave(&db, id).await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "pick_wave_id": result.wave.id,
            "status": result.wave.status,
            "lines": result.lines.iter().map(|line| json!({
                "id": line.id,
                "item_id": line.item_id,
                "location_id": line.location_id,
                "quantity": line.quantity,
                "status": line.status
            })).collect::<Vec<_>>(),
            "reservations": result.reservations.iter().map(|res| json!({
                "id": res.id,
                "item_id": res.item_id,
                "location_id": res.location_id,
                "quantity": res.quantity
            })).collect::<Vec<_>>(),
            "updated_at": result.wave.updated_at
        })),
    ))
}
