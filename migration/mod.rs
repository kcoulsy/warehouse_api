pub use sea_orm_migration::prelude::*;

mod m20260115_214134_create_warehouse_table;
mod m20260116_083021_create_location_table;
mod m20260116_084312_create_item_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260115_214134_create_warehouse_table::Migration),
            Box::new(m20260116_083021_create_location_table::Migration),
            Box::new(m20260116_084312_create_item_table::Migration),
        ]
    }
}
