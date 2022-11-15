use sea_orm::{entity::ActiveValue, ActiveModelTrait, DatabaseConnection};
use crate::entity;
use crate::entity::film::UpdateModel as Film;

pub async fn load(conn: &DatabaseConnection) {
    let films = vec![
        Film {
            title: "Film 1".to_string(),
            category_id:  1
        },
        Film {
            title: "Film 2".to_string(),
            category_id:  1
        },
        Film {
            title: "Film 3".to_string(),
            category_id:  2
        }
    ];

    for film in films {
        let _err = entity::film::ActiveModel {
            title: ActiveValue::Set(film.title),
            category_id: ActiveValue::Set(film.category_id),
            ..Default::default()
        }
        .insert(conn)
        .await;
    }

    println!("Film fixtures has been loaded");
}