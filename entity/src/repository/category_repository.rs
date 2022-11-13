use sea_orm::{
    entity::ActiveValue, ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait,
    IntoActiveModel, ModelTrait,
};
use crate::entity;
use crate::entity::category::{Model, Entity, UpdateModel};

pub async fn create(label: String, conn: &DatabaseConnection) -> Result<Model, DbErr> {
    entity::category::ActiveModel {
        label: ActiveValue::Set(label),
        ..Default::default()
    }
    .insert(conn)
    .await
}

pub async fn find(id: i32, conn: &DatabaseConnection) -> Result<Option<Model>, DbErr> {
    Entity::find_by_id(id).one(conn).await
}

pub async fn find_all(conn: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    Entity::find().all(conn).await
}

pub async fn update(
    id: i32,
    category_request: UpdateModel,
    conn: &DatabaseConnection,
) -> Result<Option<Model>, DbErr> {
    match find(id, conn).await? {
        Some(category_db) => {
            let mut active_model = category_db.into_active_model();
            active_model.label = ActiveValue::Set(category_request.label.to_owned());
            Ok(Some(active_model.update(conn).await?))
        }
        None => Ok(None),
    }
}

pub async fn delete(id: i32, conn: &DatabaseConnection) -> Result<Option<DeleteResult>, DbErr> {
    match Entity::find_by_id(id).one(conn).await? {
        Some(entity) => Ok(Some(entity.delete(conn).await?)),
        None => Ok(None),
    }
}
