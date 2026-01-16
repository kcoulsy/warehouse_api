use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Reservation::Table)
                    .if_not_exists()
                    .col(pk_auto(Reservation::Id))
                    .col(integer(Reservation::ItemId).not_null())
                    .col(integer(Reservation::LocationId).not_null())
                    .col(integer(Reservation::Quantity).not_null())
                    .col(timestamp_with_time_zone(Reservation::ExpiresAt))
                    .col(string(Reservation::Reason))
                    .col(timestamp_with_time_zone(Reservation::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_reservation_item")
                            .from(Reservation::Table, Reservation::ItemId)
                            .to(Item::Table, Item::Id),
                    )
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_reservation_location")
                            .from(Reservation::Table, Reservation::LocationId)
                            .to(Location::Table, Location::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Reservation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Reservation {
    Table,
    Id,
    ItemId,
    LocationId,
    Quantity,
    ExpiresAt,
    Reason,
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
