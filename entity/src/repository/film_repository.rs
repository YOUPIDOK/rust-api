use sea_orm::{
    entity::ActiveValue, ActiveModelTrait, DatabaseConnection, DbErr, DeleteResult, EntityTrait,
    IntoActiveModel, ModelTrait,
};
use crate::entity;
use crate::entity::film::{Model, Entity, UpdateModel};

pub async fn create(title: String, category_id: i32, conn: &DatabaseConnection) -> Result<Model, DbErr> {
    entity::film::ActiveModel {
        title: ActiveValue::Set(title),
        category_id: ActiveValue::Set(category_id),
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
    film_request: UpdateModel,
    conn: &DatabaseConnection,
) -> Result<Option<Model>, DbErr> {
    match find(id, conn).await? {
        Some(category_db) => {
            let mut active_model = category_db.into_active_model();
            active_model.title = ActiveValue::Set(film_request.title.to_owned());
            active_model.category_id = ActiveValue::Set(film_request.category_id.to_owned());
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
