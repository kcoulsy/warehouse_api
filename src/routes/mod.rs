use crate::db::DatabaseConnection;
use crate::utils::error::AppError;
use axum::Router;

mod v1;

pub fn create_router(db: DatabaseConnection) -> Router {
    Router::new()
        .nest("/v1", v1::create_v1_router(db))
        .fallback(handle_404)
}

async fn handle_404() -> AppError {
    AppError::not_found("The requested resource was not found")
}
