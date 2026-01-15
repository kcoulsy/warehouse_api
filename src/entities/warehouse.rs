use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "warehouse")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub address: String,
    pub timezone: String,
    #[sea_orm(column_name = "is_active")]
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWarehouse {
    pub name: String,
    pub address: String,
    pub timezone: String,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWarehouse {
    pub name: Option<String>,
    pub address: Option<String>,
    pub timezone: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
