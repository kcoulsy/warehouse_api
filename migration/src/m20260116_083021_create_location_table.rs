use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Location::Table)
                    .if_not_exists()
                    .col(pk_auto(Location::Id))
                    .col(integer(Location::WarehouseId).not_null())
                    .col(string(Location::Code).not_null())
                    .col(string(Location::Aisle).not_null())
                    .col(string(Location::Bin).not_null())
                    .col(string(Location::Shelf).not_null())
                    .col(boolean(Location::IsPickable).not_null().default(false))
                    .col(boolean(Location::IsBulk).not_null().default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Location::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Location {
    Table,
    Id,
    WarehouseId,
    Code,
    Aisle,
    Bin,
    Shelf,
    IsPickable,
    IsBulk,
}
