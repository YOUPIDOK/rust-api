use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "films")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub category_id: i32
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateModel {
    pub title: String,
    pub category_id: i32
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryId",
        to = "super::category::Column::Id"
    )]
    Category,
}

impl Related<super::category::Entity> for Entity {
  fn to() -> RelationDef {
      Relation::Category.def()
  }
}