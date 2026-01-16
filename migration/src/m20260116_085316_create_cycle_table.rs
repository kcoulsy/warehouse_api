use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create cycle_count table
        manager
            .create_table(
                Table::create()
                    .table(CycleCount::Table)
                    .if_not_exists()
                    .col(pk_auto(CycleCount::Id))
                    .col(integer(CycleCount::LocationId).not_null())
                    .col(string(CycleCount::Status).not_null())
                    .col(timestamp_with_time_zone(CycleCount::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(timestamp_with_time_zone(CycleCount::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_cycle_count_location")
                            .from(CycleCount::Table, CycleCount::LocationId)
                            .to(Location::Table, Location::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Create cycle_count_line table
        manager
            .create_table(
                Table::create()
                    .table(CycleCountLine::Table)
                    .if_not_exists()
                    .col(pk_auto(CycleCountLine::Id))
                    .col(integer(CycleCountLine::CycleCountId).not_null())
                    .col(integer(CycleCountLine::ItemId).not_null())
                    .col(integer(CycleCountLine::ExpectedQuantity).not_null())
                    .col(integer(CycleCountLine::CountedQuantity).not_null())
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_cycle_count_line_cycle_count")
                            .from(CycleCountLine::Table, CycleCountLine::CycleCountId)
                            .to(CycleCount::Table, CycleCount::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_cycle_count_line_item")
                            .from(CycleCountLine::Table, CycleCountLine::ItemId)
                            .to(Item::Table, Item::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CycleCountLine::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(CycleCount::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum CycleCount {
    Table,
    Id,
    LocationId,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum CycleCountLine {
    Table,
    Id,
    CycleCountId,
    ItemId,
    ExpectedQuantity,
    CountedQuantity,
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
