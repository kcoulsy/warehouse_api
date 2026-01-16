pub use sea_orm_migration::prelude::*;

mod m20260115_214134_create_warehouse_table;
mod m20260116_083021_create_location_table;
mod m20260116_084312_create_item_table;
mod m20260116_085256_create_ledger_table;
mod m20260116_085306_create_transfer_table;
mod m20260116_085316_create_cycle_table;
mod m20260116_085326_create_pick_table;
mod m20260116_085335_create_reservation_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260115_214134_create_warehouse_table::Migration),
            Box::new(m20260116_083021_create_location_table::Migration),
            Box::new(m20260116_084312_create_item_table::Migration),
            Box::new(m20260116_085256_create_ledger_table::Migration),
            Box::new(m20260116_085306_create_transfer_table::Migration),
            Box::new(m20260116_085316_create_cycle_table::Migration),
            Box::new(m20260116_085326_create_pick_table::Migration),
            Box::new(m20260116_085335_create_reservation_table::Migration),
        ]
    }
}
