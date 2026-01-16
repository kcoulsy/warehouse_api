use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create pick_wave table
        manager
            .create_table(
                Table::create()
                    .table(PickWave::Table)
                    .if_not_exists()
                    .col(pk_auto(PickWave::Id))
                    .col(string(PickWave::Status).not_null())
                    .col(timestamp_with_time_zone(PickWave::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone(PickWave::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        // Create pick_line table
        manager
            .create_table(
                Table::create()
                    .table(PickLine::Table)
                    .if_not_exists()
                    .col(pk_auto(PickLine::Id))
                    .col(integer(PickLine::WaveId).not_null())
                    .col(integer(PickLine::ItemId).not_null())
                    .col(integer(PickLine::LocationId).not_null())
                    .col(integer(PickLine::Quantity).not_null())
                    .col(string(PickLine::Status).not_null())
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_pick_line_wave")
                            .from(PickLine::Table, PickLine::WaveId)
                            .to(PickWave::Table, PickWave::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_pick_line_item")
                            .from(PickLine::Table, PickLine::ItemId)
                            .to(Item::Table, Item::Id),
                    )
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_pick_line_location")
                            .from(PickLine::Table, PickLine::LocationId)
                            .to(Location::Table, Location::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PickLine::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(PickWave::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PickWave {
    Table,
    Id,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum PickLine {
    Table,
    Id,
    WaveId,
    ItemId,
    LocationId,
    Quantity,
    Status,
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
