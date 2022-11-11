use actix_web::{HttpResponse, Responder, web};
use entity::entity::film::Model as Film;
use crate::services::app_state::AppState;

// pub async fn create() -> impl Responder {
//     println!("film_controller::create()");
// }

// pub async fn find() -> impl Responder {
//     // let id_i32 = id.into_inner();
//     println!("film_controller::find({})", &id_i32);
// }

// pub async fn find_by_categoory() -> impl Responder {
//     println!("film_controller::find_by_catgeory("{}", category_id_i32)");
// }

// pub async fn find_all() -> impl Responder {
//     println!("film_controller::find_all()");
// }

// pub async fn update() -> impl Responder {
//     // let id_i32 = id.into_inner();
//     println!("film_controller::update({})", &id_i32);
// }

// pub async fn delete() -> impl Responder {
//     // let id_i32 = id.into_inner();
//     println!("film_controller::delete({})", &id_i32);
// }

