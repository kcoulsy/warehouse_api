pub mod health;
pub mod item;
pub mod v1;

pub use health::health_check;
pub use item::{create_item, delete_item, get_item, get_items, update_item};
// Re-export v1 handlers for backward compatibility
pub use v1::location::{
    create_location, delete_location, get_location, get_locations, update_location,
};
pub use v1::warehouse::{
    create_warehouse, delete_warehouse, get_warehouse, get_warehouses, update_warehouse,
};
