use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create transfer table
        manager
            .create_table(
                Table::create()
                    .table(Transfer::Table)
                    .if_not_exists()
                    .col(pk_auto(Transfer::Id))
                    .col(integer(Transfer::FromLocationId).not_null())
                    .col(integer(Transfer::ToLocationId).not_null())
                    .col(string(Transfer::Status).not_null())
                    .col(timestamp_with_time_zone(Transfer::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone(Transfer::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_transfer_from_location")
                            .from(Transfer::Table, Transfer::FromLocationId)
                            .to(Location::Table, Location::Id),
                    )
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_transfer_to_location")
                            .from(Transfer::Table, Transfer::ToLocationId)
                            .to(Location::Table, Location::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Create transfer_line table
        manager
            .create_table(
                Table::create()
                    .table(TransferLine::Table)
                    .if_not_exists()
                    .col(pk_auto(TransferLine::Id))
                    .col(integer(TransferLine::TransferId).not_null())
                    .col(integer(TransferLine::ItemId).not_null())
                    .col(integer(TransferLine::Quantity).not_null())
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_transfer_line_transfer")
                            .from(TransferLine::Table, TransferLine::TransferId)
                            .to(Transfer::Table, Transfer::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_transfer_line_item")
                            .from(TransferLine::Table, TransferLine::ItemId)
                            .to(Item::Table, Item::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TransferLine::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Transfer::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Transfer {
    Table,
    Id,
    FromLocationId,
    ToLocationId,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum TransferLine {
    Table,
    Id,
    TransferId,
    ItemId,
    Quantity,
}

#[derive(DeriveIden)]
enum Location {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Item {
    Table,
    Id,
}
