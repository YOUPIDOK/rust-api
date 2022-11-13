use sea_orm::{
    entity::ActiveValue, ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult,
    IntoActiveModel, ModelTrait,
};
use crate::entity;
use crate::entity::category::{Model, Entity, UpdateModel};

pub async fn load(conn: &DatabaseConnection) {
    let categopries = vec![
      UpdateModel {
          label : "Category 1".to_string(),
      },
      UpdateModel {
          label : "Category 2".to_string(),
      } ,
      UpdateModel {
          label : "Category 3".to_string(),
      }
    ];

    for category in categopries {
        let err = entity::category::ActiveModel {
            label: ActiveValue::Set(category.label),
            ..Default::default()
        }
        .insert(conn)
        .await;
    }

    println!("Category fixtures has been loaded");
}