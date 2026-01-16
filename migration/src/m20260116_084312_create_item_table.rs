use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Item::Table)
                    .if_not_exists()
                    .col(pk_auto(Item::Id))
                    .col(string(Item::Sku).not_null())
                    .col(string(Item::Name).not_null())
                    .col(string(Item::UnitOfMeasure).not_null())
                    .col(string(Item::Barcode))
                    .col(boolean(Item::IsSerialized).not_null().default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Item::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Item {
    Table,
    Id,
    Sku,
    Name,
    UnitOfMeasure,
    Barcode,
    IsSerialized,
}
