use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "transfer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub from_location_id: i32,
    pub to_location_id: i32,
    pub status: String, // DRAFT | IN_TRANSIT | COMPLETED | CANCELLED
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
