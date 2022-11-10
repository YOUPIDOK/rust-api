use actix_web::{App, HttpServer, web};
use crate::controllers::{category_controller, film_controller};
use crate::services;

#[actix_web::main]
pub async fn start(addrs :&str) -> std::io::Result<()> {
    let state = services::app_state::init().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(categories_config)
            .configure(film_config)
    })
    .bind(addrs)?
    .run()
    .await
}

fn categories_config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/categories", web::get().to(category_controller::all))
        .route("/categories", web::post().to(category_controller::create))
        .route("/categories/{id}", web::get().to(category_controller::find))
        .route("/categories/{id}", web::patch().to(category_controller::update))
        .route("/categories/{id}", web::delete().to(category_controller::delete));
}

fn film_config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/films", web::get().to(film_controller::all))
        .route("/films", web::post().to(film_controller::create))
        .route("/films/{id}", web::get().to(film_controller::find))
        .route("/films/{id}", web::patch().to(film_controller::update))
        .route("/films/{id}", web::delete().to(film_controller::delete));
}