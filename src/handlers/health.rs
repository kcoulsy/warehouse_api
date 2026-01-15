use axum::{http::StatusCode, response::IntoResponse};
use serde_json::json;

pub async fn health_check() -> impl IntoResponse {
    (
        StatusCode::OK,
        axum::Json(json!({
            "status": "healthy",
            "service": "warehouse_api",
            "version": env!("CARGO_PKG_VERSION")
        })),
    )
}
