use sea_orm::{entity::ActiveValue, ActiveModelTrait, DatabaseConnection};
use crate::entity;
use crate::entity::category::UpdateModel as Category;

pub async fn load(conn: &DatabaseConnection) {
    let categopries = vec![
      Category {
          label : "Category 1".to_string(),
      },
      Category {
          label : "Category 2".to_string(),
      } ,
      Category {
          label : "Category 3".to_string(),
      }
    ];

    for category in categopries {
        let _err = entity::category::ActiveModel {
            label: ActiveValue::Set(category.label),
            ..Default::default()
        }
        .insert(conn)
        .await;
    }

    println!("Category fixtures has been loaded");
}