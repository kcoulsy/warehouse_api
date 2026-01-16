mod health;
mod item;
mod location;
mod pick;
mod receipt;
mod transfer;
mod warehouse;

use crate::db::DatabaseConnection;
use axum::Router;

pub fn create_v1_router(db: DatabaseConnection) -> Router {
    Router::new()
        .merge(health::health_routes())
        .merge(warehouse::warehouse_routes(db.clone()))
        .merge(location::location_routes(db.clone()))
        .merge(item::item_routes(db.clone()))
        .merge(receipt::receipt_routes(db.clone()))
        .merge(transfer::transfer_routes(db.clone()))
        .merge(pick::pick_routes(db))
}
