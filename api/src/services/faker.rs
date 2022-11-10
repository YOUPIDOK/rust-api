use entity::category::Model as Category;
use entity::film::Film;

/*** Categories ***/
pub fn categories() -> Vec<Category> {
    let action = Category {
        id: 1,
        label: "Action".to_string(),
    };

    let horror = Category {
        id: 2,
        label: "Horror".to_string(),
    };

    let sf = Category {
        id: 3,
        label: "Science fiction".to_string(),
    };

    return vec![action, horror, sf];
}

pub fn category(id: &i32) -> Option<Category> {
    for category in categories() {
        if category.id == *id {
            return Some(category);
        }
    }

    return None;
}

/*** Films ***/
pub fn films() -> Vec<Film> {
    let fast_and_furious = Film {
        id: Some(1),
        title: "Fast & Furious".to_string(),
        category_id: 1,
    };

    let ouija = Film {
        id: Some(2),
        title: "Ouija".to_string(),
        category_id: 2,
    };

    let joker = Film {
        id: Some(3),
        title: "Joker".to_string(),
        category_id: 1
    };

    return vec![fast_and_furious, ouija, joker];
}

pub fn film(id: &i32) -> Option<Film> {
    for film in films() {
        if film.id == Some(*id) {
            return Some(film);
        }
    }

    return None;
}