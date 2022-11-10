use actix_web::{HttpResponse, Responder, web};
use rand::Rng;
use services::faker;
use entity::film::Film;
use crate::services;

pub async fn all() -> impl Responder {
    HttpResponse::Ok().json(faker::films())
}

pub async fn create(mut film: web::Json<Film>) -> impl Responder {
    match faker::category(&film.category_id) {
        Some(_) => {
            let mut rng = rand::thread_rng();
            film.id = Some(rng.gen_range(1..10000));

           return HttpResponse::Ok().json(film)
        },
        None => return HttpResponse::BadRequest().body("Le film est mal construit.")
    }
}

pub async fn find(id: web::Path<i32>) -> impl Responder {
    match faker::film(&id.into_inner()) {
        Some(film) => return HttpResponse::Ok().json(film),
        None => return HttpResponse::NotFound().body("La catégorie n'existe pas pas.")
    }
}

pub async fn update(id: web::Path<i32>, mut film: web::Json<Film>) -> impl Responder {
      match faker::film(&id.into_inner().clone()) {
            Some(f) => {
                 match faker::category(&film.category_id) {
                     Some(_) => {
                         film.id = f.id;
                         return HttpResponse::Ok().json(film)
                     },
                     None => return HttpResponse::BadRequest().body("La catégorie n'existe pas pas.")
                 }
            },
            None => return HttpResponse::NotFound().body("Le film n'existe pas.")
      }
}

pub async fn delete(id: web::Path<i32>) -> impl Responder {
    match faker::film(&id.into_inner()) {
        Some(_) => return HttpResponse::Ok().finish(),
        None => return HttpResponse::NotFound().body("Le film n'existe pas")
    }
}

