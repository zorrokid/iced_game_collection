use std::env;

use dotenvy::dotenv;
use sqlx::{Pool, Sqlite, SqlitePool};

pub async fn get_db_pool(db_file: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    /*if !Path::new(db_file).exists() {
        std::fs::File::create(db_file).expect("Failed to create database file");
    }*/
    SqlitePool::connect(&db_url).await
}

pub async fn get_memory_db_pool() -> Result<Pool<Sqlite>, sqlx::Error> {
    SqlitePool::connect("sqlite::memory:").await
}
