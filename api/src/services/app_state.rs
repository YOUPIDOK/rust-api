use sea_orm::{DatabaseConnection, Database};

#[derive(Debug, Clone)]
pub struct AppState {
    pub(crate) conn: DatabaseConnection,
}

pub async fn init() -> AppState{
    AppState {
        conn: Database::connect("sqlite::memory:").await.unwrap()
    }
}
