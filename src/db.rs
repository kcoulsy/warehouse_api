pub use sea_orm::DatabaseConnection;
use sea_orm::{Database, DbErr};

pub async fn create_connection(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(database_url).await
}
