mod health;
mod location;
mod warehouse;

use crate::db::DatabaseConnection;
use axum::Router;

pub fn create_v1_router(db: DatabaseConnection) -> Router {
    Router::new()
        .merge(health::health_routes())
        .merge(warehouse::warehouse_routes(db.clone()))
        .merge(location::location_routes(db))
}
