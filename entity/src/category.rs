use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

pub const TABLE: &str = "categories";

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "categories")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub label: String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}