use axum::Router;
use crate::handlers;
use crate::error::AppError;

pub fn create_router() -> Router {
    Router::new()
        .merge(health_routes())
        .fallback(handle_404)
}

fn health_routes() -> Router {
    Router::new()
        .route("/health", axum::routing::get(handlers::health_check))
}

async fn handle_404() -> AppError {
    AppError::not_found("The requested resource was not found")
}
