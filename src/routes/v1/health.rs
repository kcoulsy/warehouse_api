use crate::handlers;
use axum::Router;

pub fn health_routes() -> Router {
    Router::new().route("/health", axum::routing::get(handlers::health_check))
}
