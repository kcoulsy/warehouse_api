use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "reservation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub item_id: i32,
    pub location_id: i32,
    pub quantity: i32,
    pub expires_at: Option<DateTimeWithTimeZone>,
    pub reason: Option<String>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
