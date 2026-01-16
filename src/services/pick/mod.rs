use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};

use crate::db::DatabaseConnection;
use crate::entities::ledger;
use crate::entities::pick;
use crate::entities::pick_line;
use crate::entities::reservation;
use crate::services::inventory;
use crate::services::receipt;
use crate::services::reservation as reservation_service;
use crate::utils::error::AppError;

/// Request item for pick wave creation
#[derive(Debug, Clone)]
pub struct PickItem {
    pub sku: String,
    pub quantity: i32,
    pub location_code: String,
}

/// Pick wave with its lines
#[derive(Debug)]
pub struct PickWaveWithLines {
    pub wave: pick::Model,
    pub lines: Vec<pick_line::Model>,
}

/// Allocated pick wave with reservations
#[derive(Debug)]
pub struct AllocatedPickWave {
    pub wave: pick::Model,
    pub lines: Vec<pick_line::Model>,
    pub reservations: Vec<reservation::Model>,
}

/// Confirmed pick with ledger entries
#[derive(Debug)]
pub struct ConfirmedPick {
    pub wave: pick::Model,
    pub lines: Vec<pick_line::Model>,
    pub ledger_entries: Vec<i32>, // ledger entry IDs
}

/// Create a pick wave with DRAFT status and pick lines
/// Validates items and locations exist, checks available stock (but doesn't reserve)
pub async fn create_pick_wave(
    db: &DatabaseConnection,
    items: Vec<PickItem>,
) -> Result<PickWaveWithLines, AppError> {
    if items.is_empty() {
        return Err(AppError::bad_request("At least one item is required"));
    }

    let mut pick_lines_data = Vec::new();
    for item_request in &items {
        if item_request.quantity <= 0 {
            return Err(AppError::bad_request(format!(
                "Quantity must be positive for item with SKU '{}'",
                item_request.sku
            )));
        }

        let item = receipt::find_item_by_sku(db, &item_request.sku)
            .await?
            .ok_or_else(|| {
                AppError::not_found(format!("Item with SKU '{}' not found", item_request.sku))
            })?;

        let location = receipt::find_location_by_code(db, &item_request.location_code)
            .await?
            .ok_or_else(|| {
                AppError::not_found(format!(
                    "Location with code '{}' not found",
                    item_request.location_code
                ))
            })?;

        // Check available stock (but don't reserve yet)
        let available = inventory::calculate_available(db, item.id, location.id).await?;
        if available < item_request.quantity {
            return Err(AppError::bad_request(format!(
                "Insufficient stock for item '{}' (SKU: {}). Available: {}, Requested: {}",
                item.name, item_request.sku, available, item_request.quantity
            )));
        }

        pick_lines_data.push((item.id, location.id, item_request.quantity));
    }

    // Create pick wave with DRAFT status
    let mut wave_model = <pick::ActiveModel as sea_orm::ActiveModelTrait>::default();
    wave_model.status = Set("DRAFT".to_string());

    let wave = wave_model
        .insert(db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to create pick wave: {}", e)))?;

    // Create pick lines with PENDING status
    let mut created_lines = Vec::new();
    for (item_id, location_id, quantity) in pick_lines_data {
        let mut line_model = <pick_line::ActiveModel as sea_orm::ActiveModelTrait>::default();
        line_model.wave_id = Set(wave.id);
        line_model.item_id = Set(item_id);
        line_model.location_id = Set(location_id);
        line_model.quantity = Set(quantity);
        line_model.status = Set("PENDING".to_string());

        let line = line_model
            .insert(db)
            .await
            .map_err(|e| AppError::internal(format!("Failed to create pick line: {}", e)))?;

        created_lines.push(line);
    }

    Ok(PickWaveWithLines {
        wave,
        lines: created_lines,
    })
}

/// Allocate inventory for a pick wave by creating reservations
/// Validates pick wave is in DRAFT status and stock is available
pub async fn allocate_pick_wave(
    db: &DatabaseConnection,
    pick_wave_id: i32,
) -> Result<AllocatedPickWave, AppError> {
    let wave = pick::Entity::find_by_id(pick_wave_id)
        .one(db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to find pick wave: {}", e)))?
        .ok_or_else(|| {
            AppError::not_found(format!("Pick wave with id {} not found", pick_wave_id))
        })?;

    if wave.status != "DRAFT" {
        return Err(AppError::bad_request(format!(
            "Pick wave with id {} is not in DRAFT status (current status: {})",
            pick_wave_id, wave.status
        )));
    }

    let lines = pick_line::Entity::find()
        .filter(pick_line::Column::WaveId.eq(pick_wave_id))
        .all(db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch pick lines: {}", e)))?;

    if lines.is_empty() {
        return Err(AppError::bad_request(format!(
            "Pick wave with id {} has no lines",
            pick_wave_id
        )));
    }

    // Check available stock and create reservations
    let mut reservations = Vec::new();
    for line in &lines {
        let available = inventory::calculate_available(db, line.item_id, line.location_id).await?;
        if available < line.quantity {
            return Err(AppError::bad_request(format!(
                "Insufficient stock for pick line {} (item_id: {}, location_id: {}). Available: {}, Requested: {}",
                line.id, line.item_id, line.location_id, available, line.quantity
            )));
        }

        let reservation = reservation_service::create_reservation(
            db,
            line.item_id,
            line.location_id,
            line.quantity,
            pick_wave_id,
        )
        .await?;

        reservations.push(reservation);
    }

    // Update pick wave status to ALLOCATED
    let mut wave_update: pick::ActiveModel = wave.clone().into();
    wave_update.status = Set("ALLOCATED".to_string());

    let updated_wave = wave_update
        .update(db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to update pick wave status: {}", e)))?;

    Ok(AllocatedPickWave {
        wave: updated_wave,
        lines,
        reservations,
    })
}

/// Confirm picks by creating ledger entries, releasing reservations, and updating statuses
/// Validates pick wave is in ALLOCATED status
pub async fn confirm_pick(
    db: &DatabaseConnection,
    pick_wave_id: i32,
) -> Result<ConfirmedPick, AppError> {
    let txn = db
        .begin()
        .await
        .map_err(|e| AppError::internal(format!("Failed to start transaction: {}", e)))?;

    let wave = pick::Entity::find_by_id(pick_wave_id)
        .one(&txn)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch pick wave: {}", e)))?
        .ok_or_else(|| {
            AppError::not_found(format!("Pick wave with id {} not found", pick_wave_id))
        })?;

    if wave.status != "ALLOCATED" {
        return Err(AppError::bad_request(format!(
            "Pick wave with id {} is not in ALLOCATED status (current status: {})",
            pick_wave_id, wave.status
        )));
    }

    let lines = pick_line::Entity::find()
        .filter(pick_line::Column::WaveId.eq(pick_wave_id))
        .all(&txn)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch pick lines: {}", e)))?;

    if lines.is_empty() {
        return Err(AppError::bad_request(format!(
            "Pick wave with id {} has no lines",
            pick_wave_id
        )));
    }

    // Create ledger entries and update line statuses
    let mut ledger_entries = Vec::new();
    let mut updated_lines = Vec::new();

    for line in &lines {
        // Calculate current on_hand before the pick
        let on_hand = inventory::calculate_on_hand(&txn, line.item_id, line.location_id).await?;
        let balance_after = on_hand - line.quantity;

        // Create ledger entry with -qty PICK
        let mut ledger_entry = <ledger::ActiveModel as sea_orm::ActiveModelTrait>::default();
        ledger_entry.item_id = Set(line.item_id);
        ledger_entry.location_id = Set(line.location_id);
        ledger_entry.quantity_change = Set(-line.quantity);
        ledger_entry.balance_after = Set(Some(balance_after));
        ledger_entry.reason_type = Set("PICK".to_string());
        ledger_entry.reference_type = Set(Some("pick_wave".to_string()));
        ledger_entry.reference_id = Set(Some(wave.id));

        let entry = ledger_entry.insert(&txn).await.map_err(|e| {
            AppError::internal(format!("Failed to create ledger entry: {}", e))
        })?;

        ledger_entries.push(entry.id);

        // Update pick line status to CONFIRMED
        let mut line_update: pick_line::ActiveModel = line.clone().into();
        line_update.status = Set("CONFIRMED".to_string());

        let updated_line = line_update
            .update(&txn)
            .await
            .map_err(|e| AppError::internal(format!("Failed to update pick line status: {}", e)))?;

        updated_lines.push(updated_line);
    }

    // Release all reservations for the pick wave
    reservation_service::release_reservations_for_pick_wave(&txn, pick_wave_id).await?;

    // Update pick wave status based on line completion
    let all_confirmed = updated_lines.iter().all(|line| line.status == "CONFIRMED");
    let new_status = if all_confirmed {
        "COMPLETED".to_string()
    } else {
        "PICKING".to_string()
    };

    let mut wave_update: pick::ActiveModel = wave.clone().into();
    wave_update.status = Set(new_status);

    let updated_wave = wave_update
        .update(&txn)
        .await
        .map_err(|e| AppError::internal(format!("Failed to update pick wave status: {}", e)))?;

    txn.commit()
        .await
        .map_err(|e| AppError::internal(format!("Failed to commit transaction: {}", e)))?;

    Ok(ConfirmedPick {
        wave: updated_wave,
        lines: updated_lines,
        ledger_entries,
    })
}
