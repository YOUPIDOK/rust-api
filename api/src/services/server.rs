use actix_web::{App, HttpServer, web};
use sea_orm::{Database, DatabaseConnection};
use crate::controllers::{category_controller, film_controller};
use crate::services;
use migration::{Migrator, MigratorTrait};

#[actix_web::main]
pub async fn start(addrs: &str, load_fixtures : bool) -> std::io::Result<()> {
    let conn = get_conn().await;

    fixtures(load_fixtures, &conn).await;

    let state = services::app_state::init(conn).await;

    println!("Server start : http://{}", addrs);

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
        .route("/categories", web::post().to(category_controller::create))
        .route("/categories/{id}", web::get().to(category_controller::find))
        .route("/categories", web::get().to(category_controller::find_all))
        .route("/categories/{id}", web::put().to(category_controller::update))
        .route("/categories/{id}", web::delete().to(category_controller::delete))
    ;
}

fn film_config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/films", web::post().to(film_controller::create))
        .route("/films/{id}", web::get().to(film_controller::find))
        .route("/films", web::get().to(film_controller::find_all))
        .route("/films/by-category/{id}", web::get().to(film_controller::find_by_category))
        .route("/films/{id}", web::patch().to(film_controller::update))
        .route("/films/{id}", web::delete().to(film_controller::delete))
    ;
}

async fn get_conn() -> DatabaseConnection {
    let conn = Database::connect("sqlite::memory:").await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    return conn;
}

async fn fixtures(load_fixtures: bool, conn: &DatabaseConnection) {
    if load_fixtures {
        entity::fixtures::category_fixtures::load(&conn).await;
        entity::fixtures::film_fixtures::load(&conn).await;
    }
}