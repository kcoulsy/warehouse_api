use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "pick_line")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub wave_id: i32,
    pub item_id: i32,
    pub location_id: i32,
    pub quantity: i32,
    pub status: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
