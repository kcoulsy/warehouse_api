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

pub async fn confirm_pick(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    if id <= 0 {
        return Err(AppError::bad_request(
            "Pick wave ID must be a positive integer",
        ));
    }

    let result = pick::confirm_pick(&db, id).await?;

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
            "ledger_entries": result.ledger_entries,
            "updated_at": result.wave.updated_at
        })),
    ))
}
