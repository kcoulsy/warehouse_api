use crate::db::DatabaseConnection;
use crate::handlers;
use axum::Router;

pub fn location_routes(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/locations", axum::routing::get(handlers::get_locations))
        .with_state(db)
}
