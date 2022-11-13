use sea_orm::{
    entity::ActiveValue, ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult,
    IntoActiveModel, ModelTrait,
};
use crate::entity;
use crate::entity::film::{Model, Entity, UpdateModel};

pub async fn load(conn: &DatabaseConnection) {
    // let films = vec![
    //
    // ];

    // for film in films {
    //     let err = entity::category::ActiveModel {
    //         // label: ActiveValue::Set(category.label),
    //         ..Default::default()
    //     }
    //     .insert(conn)
    //     .await;
    // }

    println!("Film fixtures has been loaded");
}