use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "cycle_count_line")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub cycle_count_id: i32,
    pub item_id: i32,
    pub expected_quantity: i32,
    pub counted_quantity: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
