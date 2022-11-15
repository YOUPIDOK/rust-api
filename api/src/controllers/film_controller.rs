use actix_web::{HttpResponse, Responder, web};
use entity::entity::film::UpdateModel as Film;
use crate::services::app_state::AppState;
use entity::repository::film_repository;

pub async fn create(film: web::Json<Film>, app_state: web::Data<AppState>) -> impl Responder {
    println!("film_controller::create()");

    match film_repository::create(film.title.to_string(), film.category_id,  &app_state.conn).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn find(
    app_state: web::Data<AppState>,
    id: web::Path<String>,
) -> impl Responder {
    let id = id.into_inner();
    println!("film_controller::find({})", &id);

     match id.parse::<i32>() {
        Ok(id) =>match film_repository::find(id, &app_state.conn).await {
            Ok(category) => match category {
                Some(film) => HttpResponse::Ok().json(film),
                None => HttpResponse::NotFound().body("Film not found"),
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string())
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string())
    }
}

pub async fn find_by_category(
    // app_state: web::Data<AppState>,
    id: web::Path<String>,
) -> impl Responder {
    let id = id.into_inner();
    println!("film_controller::find_by_catgeory({})", id);
    HttpResponse::Ok().finish()
    // TODO
}

pub async fn find_all(app_state: web::Data<AppState>) -> impl Responder {
    println!("film_controller::find_all()");
    match film_repository::find_all(&app_state.conn).await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn update(
    app_state: web::Data<AppState>,
    id: web::Path<String>,
    catgeory: web::Json<Film>,
) -> impl Responder {
    let id = id.into_inner();
    println!("film_controller::update({})", &id);

     match id.parse::<i32>() {
        Ok(id) => match film_repository::update(id,catgeory.into_inner(), &app_state.conn ).await {
            Ok(film_option) => match film_option {
                Some(film) => HttpResponse::Ok().json(film),
                None => HttpResponse::NotFound().body("Film not found"),
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string())
    }
}

pub async fn delete(id: web::Path<String>, app_state: web::Data<AppState>) -> impl Responder {
    let id = id.into_inner();
    println!("film_controller::delete({})", &id);

    match id.parse::<i32>() {
        Ok(id) => match film_repository::delete(id, &app_state.conn).await {
            Ok(res) => match res {
                Some(res) => {
                    HttpResponse::Ok().body(format!("{} row deleted", res.rows_affected))
                }
                None => HttpResponse::NotFound().body("No film deleted"),
            },
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
        Err(err) => HttpResponse::NotFound().body(err.to_string())
    }
}
