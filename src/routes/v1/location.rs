use crate::db::DatabaseConnection;
use crate::handlers;
use axum::Router;

pub fn location_routes(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/locations", axum::routing::get(handlers::get_locations))
        .route("/locations", axum::routing::post(handlers::create_location))
        .route("/locations/:id", axum::routing::get(handlers::get_location))
        .route(
            "/locations/:id",
            axum::routing::put(handlers::update_location),
        )
        .route(
            "/locations/:id",
            axum::routing::delete(handlers::delete_location),
        )
        .with_state(db)
}
