use axum::{http::StatusCode, response::IntoResponse};
use serde_json::json;

pub async fn get_warehouses() -> impl IntoResponse {
    (
        StatusCode::OK,
        axum::Json(json!({
            "warehouses": []
        })),
    )
}
