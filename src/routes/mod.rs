use axum::Router;
use crate::handlers;

pub fn create_router() -> Router {
    Router::new()
        .merge(health_routes())
}

fn health_routes() -> Router {
    Router::new()
        .route("/health", axum::routing::get(handlers::health_check))
}
