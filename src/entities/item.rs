use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "item")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub sku: String,
    pub name: String,
    pub unit_of_measure: String,
    pub barcode: Option<String>,
    pub is_serialized: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateItem {
    pub sku: String,
    pub name: String,
    pub unit_of_measure: String,
    pub barcode: Option<String>,
    pub is_serialized: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateItem {
    pub sku: Option<String>,
    pub name: Option<String>,
    pub unit_of_measure: Option<String>,
    pub barcode: Option<String>,
    pub is_serialized: Option<bool>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
