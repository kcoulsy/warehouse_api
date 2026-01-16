use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "location")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub warehouse_id: i32,
    pub code: String,
    pub aisle: String,
    pub bin: String,
    pub shelf: String,
    pub is_pickable: bool,
    pub is_bulk: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLocation {
    pub warehouse_id: i32,
    pub code: String,
    pub aisle: String,
    pub bin: String,
    pub shelf: String,
    pub is_pickable: bool,
    pub is_bulk: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateLocation {
    pub warehouse_id: Option<i32>,
    pub code: Option<String>,
    pub aisle: Option<String>,
    pub bin: Option<String>,
    pub shelf: Option<String>,
    pub is_pickable: Option<bool>,
    pub is_bulk: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
