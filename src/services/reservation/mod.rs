use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set};

use crate::db::DatabaseConnection;
use crate::entities::reservation;
use crate::utils::error::AppError;

/// Create a reservation for a pick wave
/// Stores pick_wave_id in reason field as "pick_wave:{id}"
pub async fn create_reservation<C: ConnectionTrait>(
    db: &C,
    item_id: i32,
    location_id: i32,
    quantity: i32,
    pick_wave_id: i32,
) -> Result<reservation::Model, AppError> {
    if quantity <= 0 {
        return Err(AppError::bad_request(
            "Reservation quantity must be positive",
        ));
    }

    let reason = format!("pick_wave:{}", pick_wave_id);

    let mut reservation_model = <reservation::ActiveModel as sea_orm::ActiveModelTrait>::default();
    reservation_model.item_id = Set(item_id);
    reservation_model.location_id = Set(location_id);
    reservation_model.quantity = Set(quantity);
    reservation_model.reason = Set(Some(reason));
    reservation_model.expires_at = Set(None); // No expiry as per plan

    let reservation = reservation_model
        .insert(db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to create reservation: {}", e)))?;

    Ok(reservation)
}

/// Release all reservations for a pick wave
pub async fn release_reservations_for_pick_wave<C: ConnectionTrait>(
    db: &C,
    pick_wave_id: i32,
) -> Result<u64, AppError> {
    let reason_pattern = format!("pick_wave:{}", pick_wave_id);

    let result = reservation::Entity::delete_many()
        .filter(reservation::Column::Reason.eq(reason_pattern.clone()))
        .exec(db)
        .await
        .map_err(|e| {
            AppError::internal(format!(
                "Failed to release reservations for pick wave {}: {}",
                pick_wave_id, e
            ))
        })?;

    Ok(result.rows_affected)
}

/// Calculate total reserved quantity for a pick wave
pub async fn calculate_reserved_for_pick_wave(
    db: &DatabaseConnection,
    pick_wave_id: i32,
) -> Result<i32, AppError> {
    let reason_pattern = format!("pick_wave:{}", pick_wave_id);

    let records = reservation::Entity::find()
        .filter(reservation::Column::Reason.eq(reason_pattern))
        .all(db)
        .await
        .map_err(|e| {
            AppError::internal(format!(
                "Failed to calculate reserved quantity for pick wave {}: {}",
                pick_wave_id, e
            ))
        })?;

    let sum: i32 = records.iter().map(|r| r.quantity).sum();
    Ok(sum)
}
