use crate::db::DatabaseConnection;
use crate::handlers;
use axum::Router;

pub fn receipt_routes(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/receipts", axum::routing::post(handlers::create_receipt))
        .route(
            "/receipts/bulk",
            axum::routing::post(handlers::bulk_receipt),
        )
        .route(
            "/receipts/generate-sample",
            axum::routing::get(handlers::generate_sample),
        )
        .with_state(db)
}
