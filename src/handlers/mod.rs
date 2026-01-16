pub mod health;
pub mod v1;

pub use health::health_check;
// Re-export v1 handlers for backward compatibility
pub use v1::item::{create_item, delete_item, get_item, get_items, update_item};
pub use v1::location::{
    create_location, delete_location, get_location, get_locations, update_location,
};
pub use v1::receipt::{bulk_receipt, create_receipt, generate_sample};
pub use v1::transfer::{complete_transfer, create_transfer};
pub use v1::warehouse::{
    create_warehouse, delete_warehouse, get_warehouse, get_warehouses, update_warehouse,
};
