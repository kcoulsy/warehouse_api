pub mod location;
pub mod warehouse;

pub use location::{
    create_location, delete_location, get_location, get_locations, update_location,
};
pub use warehouse::{
    create_warehouse, delete_warehouse, get_warehouse, get_warehouses, update_warehouse,
};
