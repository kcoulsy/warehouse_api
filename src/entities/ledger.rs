use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "inventory_ledger")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub item_id: i32,
    pub location_id: i32,
    pub quantity_change: i32,           // +10, -3
    pub balance_after: Option<i32>,     // optional (cached)
    pub reason_type: String,            // RECEIPT | PICK | TRANSFER | ADJUSTMENT | COUNT
    pub reference_type: Option<String>, // order_id, transfer_id, count_id
    pub reference_id: Option<i32>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
