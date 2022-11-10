use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Film {
    pub id: Option<i32>,
    pub title: String,
    pub category_id: i32
}