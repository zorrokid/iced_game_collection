use std::{collections::HashMap, sync::Arc};

use sqlx::{Pool, Sqlite};

use super::database_error::DatabaseError;

pub trait SettingReadRepository {
    async fn get_settings(&self) -> Result<HashMap<String, String>, DatabaseError>;
    async fn get_setting(&self, key: &str) -> Result<String, DatabaseError>;
}

pub trait SettingWriteRepository {
    async fn add_setting(&self, key: &str, value: &str) -> Result<(), DatabaseError>;
    async fn update_setting(&self, key: &str, value: &str) -> Result<(), DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct SettingRepository {
    pool: Arc<Pool<Sqlite>>,
}

impl SettingRepository {
    pub fn new(pool: Arc<Pool<Sqlite>>) -> Self {
        Self { pool }
    }
}

impl SettingReadRepository for SettingRepository {
    async fn get_settings(&self) -> Result<HashMap<String, String>, DatabaseError> {
        let rows = sqlx::query!("SELECT key, value FROM setting")
            .fetch_all(&*self.pool)
            .await?;
        let settings = rows.into_iter().map(|row| (row.key, row.value)).collect();

        Ok(settings)
    }

    async fn get_setting(&self, key: &str) -> Result<String, DatabaseError> {
        let row = sqlx::query!("SELECT value FROM setting WHERE key = ?", key)
            .fetch_one(&*self.pool)
            .await?;
        Ok(row.value)
    }
}

impl SettingWriteRepository for SettingRepository {
    async fn add_setting(&self, key: &str, value: &str) -> Result<(), DatabaseError> {
        sqlx::query!(
            "INSERT INTO setting (key, value) 
             VALUES (?, ?)
             ",
            key,
            value
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }
    async fn update_setting(&self, key: &str, value: &str) -> Result<(), DatabaseError> {
        sqlx::query!(
            "UPDATE setting SET value = ? 
             WHERE key = ?
            ",
            value,
            key
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::database::database_with_sqlx::get_memory_db_pool;

    use super::*;
    use sqlx::query;

    #[async_std::test]
    async fn test_get_settings() {
        let pool = get_memory_db_pool().await.unwrap();
        query!(
            "INSERT INTO setting (key, value) VALUES (?, ?)",
            "test_key",
            "test_value"
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = SettingsRepository { pool }.get_settings().await.unwrap();
        let mut expected = HashMap::new();
        expected.insert("test_key".to_string(), "test_value".to_string());
        assert_eq!(result, expected);
    }

    #[async_std::test]
    async fn test_get_setting() {
        let pool = get_memory_db_pool().await.unwrap();
        query!(
            "INSERT INTO setting (key, value) VALUES (?, ?)",
            "test_key",
            "test_value"
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = SettingsRepository { pool }
            .get_setting("test_key")
            .await
            .unwrap();
        assert_eq!(result, "test_value");
    }

    #[async_std::test]
    async fn test_add_setting() {
        let pool = get_memory_db_pool().await.unwrap();
        let key = "test_key";
        let value = "test_value";
        let repository = SettingsRepository { pool };
        repository.add_setting(key, value).await.unwrap();

        let result = repository.get_setting(key).await.unwrap();
        assert_eq!(result, value);
    }

    #[async_std::test]
    async fn test_update_setting() {
        let pool = get_memory_db_pool().await.unwrap();
        let key = "test_key";
        let value = "test_value";
        let repository = SettingsRepository { pool };
        repository.add_setting(key, value).await.unwrap();

        let new_value = "new_test_value";
        repository.update_setting(key, new_value).await.unwrap();

        let result = repository.get_setting(key).await.unwrap();
        assert_eq!(result, new_value);
    }
}
