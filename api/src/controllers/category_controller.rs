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
    id: web::Path<String>,
) -> impl Responder {
    let id = id.into_inner();
    println!("category_controller::find({})", &id);

     match id.parse::<i32>() {
        Ok(id) =>match category_repository::find(id, &app_state.conn).await {
            Ok(category) => match category {
                Some(category) => HttpResponse::Ok().json(category),
                None => HttpResponse::NotFound().body("Category not found"),
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string())
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string())
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
    id: web::Path<String>,
    catgeory: web::Json<UpdateModel>,
) -> impl Responder {
    let id = id.into_inner();
    println!("category_controller::update({})", &id);

     match id.parse::<i32>() {
        Ok(id) => match category_repository::update(id,catgeory.into_inner(), &app_state.conn ).await {
            Ok(user_option) => match user_option {
                Some(user) => HttpResponse::Ok().json(user),
                None => HttpResponse::NotFound().body("Category not found"),
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string())
    }
}

pub async fn delete(id: web::Path<String>, app_state: web::Data<AppState>) -> impl Responder {
    let id = id.into_inner();
    println!("category_controller::delete({})", &id);

    match id.parse::<i32>() {
        Ok(id) => match category_repository::delete(id, &app_state.conn).await {
            Ok(res) => match res {
                Some(res) => {
                    HttpResponse::Ok().body(format!("{} row deleted", res.rows_affected))
                }
                None => HttpResponse::NotFound().body("No category deleted"),
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string())
    }
}