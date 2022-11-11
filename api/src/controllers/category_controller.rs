use actix_web::{HttpResponse, Responder, web};
use entity::entity::category::UpdateModel;
use entity::repository::category_repository;
use crate::services::app_state::AppState;

pub async fn create(category: web::Json<UpdateModel>, app_state: web::Data<AppState>) -> impl Responder {
    println!("category_controller::create()");

    match category_repository::create(category.label.to_string(), &app_state.conn).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn find(
    app_state: web::Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    let id_i32 = id.into_inner();
    println!("category_controller::find({})", &id_i32);

    match category_repository::find(id_i32, &app_state.conn).await {
        Ok(category) => match category {
            Some(category) => HttpResponse::Ok().json(category),
            None => HttpResponse::NotFound().body("Category not found"),
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }

}

pub async fn find_all(app_state: web::Data<AppState>) -> impl Responder {
    println!("category_controller::find_all()");

    match category_repository::find_all(&app_state.conn).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<i32>,
    catgeory: web::Json<UpdateModel>,
) -> impl Responder {
    let id_i32 = id.into_inner();
    println!("category_controller::update({})", &id_i32);

   match category_repository::update(id_i32,catgeory.into_inner(), &app_state.conn ).await {
        Ok(user_option) => match user_option {
            Some(user) => HttpResponse::Ok().json(user),
            None => HttpResponse::NotFound().body("Category not found"),
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// pub async fn delete(id: web::Path<i32>) -> impl Responder {
//     let id_i32 = id.into_inner();
//     println!("category_controller::delete({})", &id_i32);
// }