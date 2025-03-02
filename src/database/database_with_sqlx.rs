use std::env;

use dotenvy::dotenv;
use sqlx::{Pool, Sqlite, SqlitePool};

pub async fn get_db_pool() -> Result<Pool<Sqlite>, sqlx::Error> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePool::connect(&db_url).await
}

pub async fn get_memory_db_pool() -> Result<Pool<Sqlite>, sqlx::Error> {
    SqlitePool::connect("sqlite::memory:").await
}

struct DatabaseWithSqlx {
    pool: Pool<Sqlite>,
}

impl DatabaseWithSqlx {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let pool = get_db_pool().await?;
        Ok(Self { pool })
    }

    pub async fn new_memory() -> Result<Self, sqlx::Error> {
        let pool = get_memory_db_pool().await?;
        Ok(Self { pool })
    }
}
