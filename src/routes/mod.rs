use axum::Router;
use crate::handlers;
use crate::error::AppError;
use crate::db::DatabaseConnection;

pub fn create_router(db: DatabaseConnection) -> Router {
    Router::new()
        .merge(health_routes())
        .merge(warehouse_routes(db))
        .fallback(handle_404)
}

fn health_routes() -> Router {
    Router::new()
        .route("/health", axum::routing::get(handlers::health_check))
}

fn warehouse_routes(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/warehouses", axum::routing::get(handlers::get_warehouses))
        .route("/warehouses", axum::routing::post(handlers::create_warehouse))
        .route("/warehouses/:id", axum::routing::get(handlers::get_warehouse))
        .route("/warehouses/:id", axum::routing::put(handlers::update_warehouse))
        .route("/warehouses/:id", axum::routing::delete(handlers::delete_warehouse))
        .with_state(db)
}

async fn handle_404() -> AppError {
    AppError::not_found("The requested resource was not found")
}
