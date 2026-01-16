use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(InventoryLedger::Table)
                    .if_not_exists()
                    .col(pk_auto(InventoryLedger::Id))
                    .col(integer(InventoryLedger::ItemId).not_null())
                    .col(integer(InventoryLedger::LocationId).not_null())
                    .col(integer(InventoryLedger::QuantityChange).not_null())
                    .col(integer(InventoryLedger::BalanceAfter))
                    .col(string(InventoryLedger::ReasonType).not_null())
                    .col(string(InventoryLedger::ReferenceType))
                    .col(integer(InventoryLedger::ReferenceId))
                    .col(
                        timestamp_with_time_zone(InventoryLedger::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_ledger_item")
                            .from(InventoryLedger::Table, InventoryLedger::ItemId)
                            .to(Item::Table, Item::Id),
                    )
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_ledger_location")
                            .from(InventoryLedger::Table, InventoryLedger::LocationId)
                            .to(Location::Table, Location::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(InventoryLedger::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum InventoryLedger {
    Table,
    Id,
    ItemId,
    LocationId,
    QuantityChange,
    BalanceAfter,
    ReasonType,
    ReferenceType,
    ReferenceId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Item {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Location {
    Table,
    Id,
}
