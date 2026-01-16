use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};

use crate::db::DatabaseConnection;
use crate::entities::ledger;
use crate::entities::location;
use crate::entities::transfer;
use crate::entities::transfer_line;
use crate::services::inventory;
use crate::services::receipt;
use crate::utils::error::AppError;

/// Create a transfer with DRAFT status and transfer lines
/// Validates stock availability before creating the transfer
pub async fn create_transfer(
    db: &DatabaseConnection,
    from_location_id: i32,
    to_location_id: i32,
    items: Vec<TransferItem>,
) -> Result<TransferWithLines, AppError> {
    location::Entity::find_by_id(from_location_id)
        .one(db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to find source location: {}", e)))?
        .ok_or_else(|| {
            AppError::not_found(format!(
                "Source location with id {} not found",
                from_location_id
            ))
        })?;

    location::Entity::find_by_id(to_location_id)
        .one(db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to find destination location: {}", e)))?
        .ok_or_else(|| {
            AppError::not_found(format!(
                "Destination location with id {} not found",
                to_location_id
            ))
        })?;

    if from_location_id == to_location_id {
        return Err(AppError::bad_request(
            "Source and destination locations must be different",
        ));
    }

    let mut transfer_lines = Vec::new();
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

        let available = inventory::calculate_available(db, item.id, from_location_id).await?;
        if available < item_request.quantity {
            return Err(AppError::bad_request(format!(
                "Insufficient stock for item '{}' (SKU: {}). Available: {}, Requested: {}",
                item.name, item_request.sku, available, item_request.quantity
            )));
        }

        transfer_lines.push((item.id, item_request.quantity));
    }

    let mut transfer_model = <transfer::ActiveModel as sea_orm::ActiveModelTrait>::default();
    transfer_model.from_location_id = Set(from_location_id);
    transfer_model.to_location_id = Set(to_location_id);
    transfer_model.status = Set("DRAFT".to_string());

    let transfer = transfer_model
        .insert(db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to create transfer: {}", e)))?;

    // Create transfer lines
    let mut created_lines = Vec::new();
    for (item_id, quantity) in transfer_lines {
        let mut line_model = <transfer_line::ActiveModel as sea_orm::ActiveModelTrait>::default();
        line_model.transfer_id = Set(transfer.id);
        line_model.item_id = Set(item_id);
        line_model.quantity = Set(quantity);

        let line = line_model
            .insert(db)
            .await
            .map_err(|e| AppError::internal(format!("Failed to create transfer line: {}", e)))?;

        created_lines.push(line);
    }

    Ok(TransferWithLines {
        transfer,
        lines: created_lines,
    })
}

pub async fn complete_transfer(
    db: &DatabaseConnection,
    transfer_id: i32,
) -> Result<CompletedTransfer, AppError> {
    let txn = db
        .begin()
        .await
        .map_err(|e| AppError::internal(format!("Failed to start transaction: {}", e)))?;

    let transfer = transfer::Entity::find_by_id(transfer_id)
        .one(&txn)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch transfer: {}", e)))?
        .ok_or_else(|| {
            AppError::not_found(format!("Transfer with id {} not found", transfer_id))
        })?;

    if transfer.status != "DRAFT" {
        return Err(AppError::bad_request(format!(
            "Transfer with id {} is not in DRAFT status (current status: {})",
            transfer_id, transfer.status
        )));
    }

    let lines = transfer_line::Entity::find()
        .filter(transfer_line::Column::TransferId.eq(transfer_id))
        .all(&txn)
        .await
        .map_err(|e| AppError::internal(format!("Failed to fetch transfer lines: {}", e)))?;

    if lines.is_empty() {
        return Err(AppError::bad_request(format!(
            "Transfer with id {} has no lines",
            transfer_id
        )));
    }

    let mut ledger_entries = Vec::new();
    for line in &lines {
        let source_on_hand =
            inventory::calculate_on_hand(&txn, line.item_id, transfer.from_location_id).await?;
        let source_balance_after = source_on_hand - line.quantity;

        let mut source_ledger = <ledger::ActiveModel as sea_orm::ActiveModelTrait>::default();
        source_ledger.item_id = Set(line.item_id);
        source_ledger.location_id = Set(transfer.from_location_id);
        source_ledger.quantity_change = Set(-line.quantity);
        source_ledger.balance_after = Set(Some(source_balance_after));
        source_ledger.reason_type = Set("TRANSFER".to_string());
        source_ledger.reference_type = Set(Some("transfer".to_string()));
        source_ledger.reference_id = Set(Some(transfer.id));

        let source_entry = source_ledger.insert(&txn).await.map_err(|e| {
            AppError::internal(format!("Failed to create source ledger entry: {}", e))
        })?;

        let dest_on_hand =
            inventory::calculate_on_hand(&txn, line.item_id, transfer.to_location_id).await?;
        let dest_balance_after = dest_on_hand + line.quantity;

        let mut dest_ledger = <ledger::ActiveModel as sea_orm::ActiveModelTrait>::default();
        dest_ledger.item_id = Set(line.item_id);
        dest_ledger.location_id = Set(transfer.to_location_id);
        dest_ledger.quantity_change = Set(line.quantity);
        dest_ledger.balance_after = Set(Some(dest_balance_after));
        dest_ledger.reason_type = Set("TRANSFER".to_string());
        dest_ledger.reference_type = Set(Some("transfer".to_string()));
        dest_ledger.reference_id = Set(Some(transfer.id));

        let dest_entry = dest_ledger.insert(&txn).await.map_err(|e| {
            AppError::internal(format!("Failed to create destination ledger entry: {}", e))
        })?;

        ledger_entries.push((source_entry.id, dest_entry.id));
    }

    let mut transfer_update: transfer::ActiveModel = transfer.clone().into();
    transfer_update.status = Set("COMPLETED".to_string());

    let updated_transfer = transfer_update
        .update(&txn)
        .await
        .map_err(|e| AppError::internal(format!("Failed to update transfer status: {}", e)))?;

    txn.commit()
        .await
        .map_err(|e| AppError::internal(format!("Failed to commit transaction: {}", e)))?;

    Ok(CompletedTransfer {
        transfer: updated_transfer,
        lines,
        ledger_entries,
    })
}

/// Request item for transfer creation
#[derive(Debug, Clone)]
pub struct TransferItem {
    pub sku: String,
    pub quantity: i32,
}

/// Transfer with its lines
#[derive(Debug)]
pub struct TransferWithLines {
    pub transfer: transfer::Model,
    pub lines: Vec<transfer_line::Model>,
}

/// Completed transfer with ledger entries
#[derive(Debug)]
pub struct CompletedTransfer {
    pub transfer: transfer::Model,
    pub lines: Vec<transfer_line::Model>,
    pub ledger_entries: Vec<(i32, i32)>, // (source_ledger_id, dest_ledger_id)
}
