use sea_orm::{
  Database,
  DatabaseConnection,
};

pub async fn init_db() -> DatabaseConnection {
  let database_url = std::env::var("DB_URL")
    .expect("DATABASE_URL must be set");

  Database::connect(database_url)
    .await
    .expect("Failed to connect to database")
}
