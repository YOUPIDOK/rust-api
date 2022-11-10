use actix_web::{HttpResponse, Responder, web};
use entity::category::Model as Category;
use rand::Rng;
use services::faker;
use crate::services;

pub async fn all() -> impl Responder {
    HttpResponse::Ok().json(faker::categories())
}

pub async fn create(mut category: web::Json<Category>) -> impl Responder {
    let mut rng = rand::thread_rng();
    category.id = rng.gen_range(1..10000);

    HttpResponse::Ok().json(category)
}

pub async fn find(id: web::Path<i32>) -> impl Responder {
    match faker::category(&id.into_inner()) {
        Some(category) => return HttpResponse::Ok().json(category),
        None => return HttpResponse::NotFound().body("La catégorie n'existe pas.")
    }
}

pub async fn update(id: web::Path<i32>,mut category: web::Json<Category>) -> impl Responder {
    match faker::category(&id.into_inner().clone()) {
        Some(c) => {
            category.id = c.id;
            return HttpResponse::Ok().json(category)
        },
        None => return HttpResponse::NotFound().body("La catégorie n'existe pas.")
    }
}

pub async fn delete(id: web::Path<i32>) -> impl Responder {
    let id_i32 = id.into_inner();

    match faker::category(&id_i32) {
        Some(_) => {
            for f in faker::films() {
               if f.category_id == id_i32 {
                    return HttpResponse::BadRequest().body("Il existe encore des films utilisant cette catégorie.")
                }
            }
            return HttpResponse::Ok().finish()
        },
        None => return HttpResponse::NotFound().body("La catégorie n'existe pas.")
    }
}