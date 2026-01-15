use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Warehouse::Table)
                    .if_not_exists()
                    .col(pk_auto(Warehouse::Id))
                    .col(string(Warehouse::Name).not_null())
                    .col(string(Warehouse::Address).not_null())
                    .col(string(Warehouse::Timezone).not_null())
                    .col(boolean(Warehouse::IsActive).not_null().default(false))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Warehouse::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Warehouse {
    Table,
    Id,
    Name,
    Address,
    Timezone,
    IsActive,
}
