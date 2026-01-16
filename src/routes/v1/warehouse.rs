use crate::db::DatabaseConnection;
use crate::handlers;
use axum::Router;

pub fn warehouse_routes(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/warehouses", axum::routing::get(handlers::get_warehouses))
        .route(
            "/warehouses",
            axum::routing::post(handlers::create_warehouse),
        )
        .route(
            "/warehouses/:id",
            axum::routing::get(handlers::get_warehouse),
        )
        .route(
            "/warehouses/:id",
            axum::routing::put(handlers::update_warehouse),
        )
        .route(
            "/warehouses/:id",
            axum::routing::delete(handlers::delete_warehouse),
        )
        .with_state(db)
}
