use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set, TransactionTrait,
};
use uuid::Uuid;

use crate::db::DatabaseConnection;
use crate::entities::item;
use crate::entities::ledger;
use crate::entities::location;
use crate::utils::error::AppError;

pub async fn find_item_by_sku<C: ConnectionTrait>(
    db: &C,
    sku: &str,
) -> Result<Option<item::Model>, AppError> {
    let item = item::Entity::find()
        .filter(item::Column::Sku.eq(sku))
        .one(db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to find item by SKU: {}", e)))?;

    Ok(item)
}

pub async fn find_location_by_code<C: ConnectionTrait>(
    db: &C,
    code: &str,
) -> Result<Option<location::Model>, AppError> {
    let location = location::Entity::find()
        .filter(location::Column::Code.eq(code))
        .one(db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to find location by code: {}", e)))?;

    Ok(location)
}

pub async fn find_or_create_item_by_sku<C: ConnectionTrait>(
    db: &C,
    sku: &str,
    name: Option<String>,
    unit_of_measure: Option<String>,
    barcode: Option<String>,
    is_serialized: Option<bool>,
) -> Result<item::Model, AppError> {
    if let Some(item) = find_item_by_sku(db, sku).await? {
        return Ok(item);
    }

    let name = name.unwrap_or_else(|| format!("Item {}", sku));
    let unit_of_measure = unit_of_measure.unwrap_or_else(|| "EA".to_string());
    let barcode = barcode.unwrap_or_else(|| sku.to_string());
    let is_serialized = is_serialized.unwrap_or(false);

    let mut active_model = <item::ActiveModel as sea_orm::ActiveModelTrait>::default();
    active_model.sku = Set(sku.to_string());
    active_model.name = Set(name);
    active_model.unit_of_measure = Set(unit_of_measure);
    active_model.barcode = Set(Some(barcode));
    active_model.is_serialized = Set(is_serialized);

    let item = active_model
        .insert(db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to create item: {}", e)))?;

    Ok(item)
}

/// Find or create a location by code. Used for CSV bulk import.
/// If location doesn't exist, creates it with provided or default values.
pub async fn find_or_create_location_by_code<C: ConnectionTrait>(
    db: &C,
    code: &str,
    warehouse_id: Option<i32>,
    aisle: Option<String>,
    bin: Option<String>,
    shelf: Option<String>,
    is_pickable: Option<bool>,
    is_bulk: Option<bool>,
) -> Result<location::Model, AppError> {
    if let Some(location) = find_location_by_code(db, code).await? {
        return Ok(location);
    }

    let warehouse_id = warehouse_id.ok_or_else(|| {
        AppError::bad_request(format!(
            "warehouse_id is required when creating new location with code: {}",
            code
        ))
    })?;

    let aisle = aisle.unwrap_or_else(|| "A".to_string());
    let bin = bin.unwrap_or_else(|| "1".to_string());
    let shelf = shelf.unwrap_or_else(|| "1".to_string());
    let is_pickable = is_pickable.unwrap_or(false);
    let is_bulk = is_bulk.unwrap_or(false);

    let mut active_model = <location::ActiveModel as sea_orm::ActiveModelTrait>::default();
    active_model.warehouse_id = Set(warehouse_id);
    active_model.code = Set(code.to_string());
    active_model.aisle = Set(aisle);
    active_model.bin = Set(bin);
    active_model.shelf = Set(shelf);
    active_model.is_pickable = Set(is_pickable);
    active_model.is_bulk = Set(is_bulk);

    let location = active_model
        .insert(db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to create location: {}", e)))?;

    Ok(location)
}

pub async fn create_receipt_ledger_entry<C: ConnectionTrait>(
    db: &C,
    item_id: i32,
    location_id: i32,
    quantity: i32,
    receipt_id: &str,
) -> Result<i32, AppError> {
    if quantity <= 0 {
        return Err(AppError::bad_request(
            "Quantity must be positive for receipts",
        ));
    }

    let current_on_hand =
        crate::services::inventory::calculate_on_hand(db, item_id, location_id).await?;

    let balance_after = current_on_hand + quantity;

    // Convert receipt_id (UUID string) to i32 for reference_id
    // We'll use a hash-based approach to convert UUID to i32
    // This is a simple approach - in production you might want a separate receipt table
    let receipt_id_hash = receipt_id
        .as_bytes()
        .iter()
        .fold(0i32, |acc, &b| acc.wrapping_mul(31).wrapping_add(b as i32));

    let mut active_model = <ledger::ActiveModel as sea_orm::ActiveModelTrait>::default();
    active_model.item_id = Set(item_id);
    active_model.location_id = Set(location_id);
    active_model.quantity_change = Set(quantity);
    active_model.balance_after = Set(Some(balance_after));
    active_model.reason_type = Set("RECEIPT".to_string());
    active_model.reference_type = Set(Some("receipt".to_string()));
    active_model.reference_id = Set(Some(receipt_id_hash));

    let ledger_entry = active_model
        .insert(db)
        .await
        .map_err(|e| AppError::internal(format!("Failed to create ledger entry: {}", e)))?;

    Ok(ledger_entry.id)
}

pub async fn process_bulk_receipt(
    db: &DatabaseConnection,
    rows: Vec<BulkReceiptRow>,
) -> Result<BulkReceiptResult, AppError> {
    let receipt_id = Uuid::new_v4().to_string();
    let total_rows = rows.len();
    let mut successful_rows = 0;
    let mut errors = Vec::new();

    let txn = db
        .begin()
        .await
        .map_err(|e| AppError::internal(format!("Failed to start transaction: {}", e)))?;

    for (index, row) in rows.iter().enumerate() {
        let row_number = index + 1;

        if row.quantity <= 0 {
            errors.push(BulkReceiptError {
                row: row_number,
                error: "Quantity must be positive".to_string(),
            });
            continue;
        }

        let item = match find_or_create_item_by_sku(
            &txn,
            &row.sku,
            row.name.clone(),
            row.unit_of_measure.clone(),
            row.barcode.clone(),
            row.is_serialized,
        )
        .await
        {
            Ok(item) => item,
            Err(e) => {
                errors.push(BulkReceiptError {
                    row: row_number,
                    error: format!("Failed to find/create item: {}", e),
                });
                continue;
            }
        };

        let location = match find_or_create_location_by_code(
            &txn,
            &row.location_code,
            row.warehouse_id,
            row.aisle.clone(),
            row.bin.clone(),
            row.shelf.clone(),
            row.is_pickable,
            row.is_bulk,
        )
        .await
        {
            Ok(location) => location,
            Err(e) => {
                errors.push(BulkReceiptError {
                    row: row_number,
                    error: format!("Failed to find/create location: {}", e),
                });
                continue;
            }
        };

        match create_receipt_ledger_entry(&txn, item.id, location.id, row.quantity, &receipt_id)
            .await
        {
            Ok(_) => {
                successful_rows += 1;
            }
            Err(e) => {
                errors.push(BulkReceiptError {
                    row: row_number,
                    error: format!("Failed to create ledger entry: {}", e),
                });
            }
        }
    }

    if !errors.is_empty() {
        txn.rollback()
            .await
            .map_err(|e| AppError::internal(format!("Failed to rollback transaction: {}", e)))?;

        return Ok(BulkReceiptResult {
            receipt_id,
            total_rows,
            successful_rows,
            errors,
        });
    }

    txn.commit()
        .await
        .map_err(|e| AppError::internal(format!("Failed to commit transaction: {}", e)))?;

    Ok(BulkReceiptResult {
        receipt_id,
        total_rows,
        successful_rows,
        errors,
    })
}

/// Represents a single row in a bulk receipt CSV
#[derive(Debug, Clone)]
pub struct BulkReceiptRow {
    pub sku: String,
    pub location_code: String,
    pub quantity: i32,
    pub name: Option<String>,
    pub unit_of_measure: Option<String>,
    pub barcode: Option<String>,
    pub is_serialized: Option<bool>,
    pub warehouse_id: Option<i32>,
    pub aisle: Option<String>,
    pub bin: Option<String>,
    pub shelf: Option<String>,
    pub is_pickable: Option<bool>,
    pub is_bulk: Option<bool>,
}

/// Result of processing a bulk receipt
#[derive(Debug)]
pub struct BulkReceiptResult {
    pub receipt_id: String,
    pub total_rows: usize,
    pub successful_rows: usize,
    pub errors: Vec<BulkReceiptError>,
}

/// Error for a specific row in bulk receipt processing
#[derive(Debug)]
pub struct BulkReceiptError {
    pub row: usize,
    pub error: String,
}
