use sea_orm::Database;
use std::env;

pub struct DatabaseHelper {
    pub db_url: String, 
}

impl DatabaseHelper { 
    pub async fn new() -> Self  {
        dotenv::dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DB URL MUST BE SET!");
        DatabaseHelper {db_url}
    }
    pub async fn connect(&self) -> Result<sea_orm::DatabaseConnection, sea_orm::DbErr> { 
        Database::connect(&self.db_url).await
    }
}