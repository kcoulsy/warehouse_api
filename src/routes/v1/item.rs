use crate::db::DatabaseConnection;
use crate::handlers;
use axum::Router;

pub fn item_routes(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/items", axum::routing::get(handlers::get_items))
        .route("/items", axum::routing::post(handlers::create_item))
        .route("/items/:id", axum::routing::get(handlers::get_item))
        .route("/items/:id", axum::routing::put(handlers::update_item))
        .route("/items/:id", axum::routing::delete(handlers::delete_item))
        .with_state(db)
}
