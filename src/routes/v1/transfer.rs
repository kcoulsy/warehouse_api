use crate::db::DatabaseConnection;
use crate::handlers;
use axum::Router;

pub fn transfer_routes(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/transfers", axum::routing::post(handlers::create_transfer))
        .route(
            "/transfers/:id/complete",
            axum::routing::post(handlers::complete_transfer),
        )
        .with_state(db)
}
