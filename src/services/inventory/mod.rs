use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::db::DatabaseConnection;
use crate::entities::ledger;
use crate::entities::reservation;
use crate::utils::error::AppError;

/// Calculate on-hand quantity by summing quantity_change from inventory_ledger
/// for a specific item and location.
///
/// Never trust a stored stock number - always compute from the ledger.
pub async fn calculate_on_hand(
    db: &DatabaseConnection,
    item_id: i32,
    location_id: i32,
) -> Result<i32, AppError> {
    let records = ledger::Entity::find()
        .filter(
            sea_orm::Condition::all()
                .add(ledger::Column::ItemId.eq(item_id))
                .add(ledger::Column::LocationId.eq(location_id)),
        )
        .all(db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to calculate on-hand quantity: {}", e)))?;

    let sum: i32 = records.iter().map(|r| r.quantity_change).sum();
    Ok(sum)
}

/// Calculate reserved quantity by summing quantity from reservations
/// for a specific item and location where the reservation hasn't expired.
pub async fn calculate_reserved(
    db: &DatabaseConnection,
    item_id: i32,
    location_id: i32,
) -> Result<i32, AppError> {
    // Get current time in UTC
    let now = chrono::Utc::now();

    let records = reservation::Entity::find()
        .filter(
            sea_orm::Condition::all()
                .add(reservation::Column::ItemId.eq(item_id))
                .add(reservation::Column::LocationId.eq(location_id))
                .add(
                    sea_orm::Condition::any()
                        .add(reservation::Column::ExpiresAt.is_null())
                        .add(reservation::Column::ExpiresAt.gt(now)),
                ),
        )
        .all(db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to calculate reserved quantity: {}", e)))?;

    let sum: i32 = records.iter().map(|r| r.quantity).sum();
    Ok(sum)
}

/// Calculate available quantity as on_hand - reserved.
pub async fn calculate_available(
    db: &DatabaseConnection,
    item_id: i32,
    location_id: i32,
) -> Result<i32, AppError> {
    let on_hand = calculate_on_hand(db, item_id, location_id).await?;
    let reserved = calculate_reserved(db, item_id, location_id).await?;

    Ok(on_hand - reserved)
}
