use crate::db::DatabaseConnection;
use crate::handlers;
use axum::Router;

pub fn pick_routes(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/pick-waves", axum::routing::post(handlers::create_pick_wave))
        .route(
            "/pick-waves/:id/allocate",
            axum::routing::post(handlers::allocate_pick_wave),
        )
        .route(
            "/pick-waves/:id/confirm-pick",
            axum::routing::post(handlers::confirm_pick),
        )
        .with_state(db)
}
