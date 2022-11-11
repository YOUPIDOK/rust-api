use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppState {
    pub(crate) conn: DatabaseConnection,
}

pub async fn init(conn: DatabaseConnection) -> AppState{
    AppState {
        conn
    }
}