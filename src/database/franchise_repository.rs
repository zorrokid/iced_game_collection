use std::sync::Arc;

use sqlx::{Pool, Sqlite};

use crate::model::model::Franchise;

use super::database_error::DatabaseError;

pub trait FranchiseReadRepository {
    async fn get_all_franchises(&self) -> Result<Vec<Franchise>, DatabaseError>;
}

pub trait FranchiseWriteRepository {
    async fn add_franchise(&self, name: &str) -> Result<i64, DatabaseError>;
    async fn update_franchise(&self, franchise: &Franchise) -> Result<i64, DatabaseError>;
    async fn delete_franchise(&self, id: i64) -> Result<(), DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct FranchiseRepository {
    pool: Arc<Pool<Sqlite>>,
}

impl FranchiseRepository {
    pub fn new(pool: Arc<Pool<Sqlite>>) -> Self {
        Self { pool }
    }
}

impl FranchiseReadRepository for FranchiseRepository {
    async fn get_all_franchises(&self) -> Result<Vec<Franchise>, DatabaseError> {
        let franchises = sqlx::query_as!(Franchise, "SELECT id, name FROM franchise")
            .fetch_all(&*self.pool)
            .await?;
        Ok(franchises)
    }
}

impl FranchiseWriteRepository for FranchiseRepository {
    async fn add_franchise(&self, name: &str) -> Result<i64, DatabaseError> {
        let result = sqlx::query!("INSERT INTO franchise (name) VALUES (?)", name)
            .execute(&*self.pool)
            .await?;
        Ok(result.last_insert_rowid())
    }

    async fn update_franchise(&self, franchise: &Franchise) -> Result<i64, DatabaseError> {
        let result = sqlx::query!(
            "UPDATE franchise SET name = ? WHERE id = ?",
            franchise.name,
            franchise.id
        )
        .execute(&*self.pool)
        .await?;
        Ok(result.last_insert_rowid())
    }

    async fn delete_franchise(&self, id: i64) -> Result<(), DatabaseError> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM software_title WHERE franchise_id = ?",
            id
        )
        .fetch_one(&*self.pool)
        .await?;
        if count > 0 {
            return Err(DatabaseError::InUse);
        }
        sqlx::query!("DELETE FROM franchise WHERE id = ?", id)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::database::{database_error::DatabaseError, database_with_sqlx::get_memory_db_pool};

    use super::*;
    use sqlx::query;

    #[async_std::test]
    async fn test_get_all_franchises() {
        let pool = get_memory_db_pool().await.unwrap();
        let franchise = Franchise {
            id: 1,
            name: "Test Franchise".to_string(),
        };
        query!(
            "INSERT INTO franchise (id, name) VALUES (?, ?)",
            franchise.id,
            franchise.name
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = FranchiseRepository { pool }
            .get_all_franchises()
            .await
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], franchise);
    }

    #[async_std::test]
    async fn test_add_franchise() {
        let pool = get_memory_db_pool().await.unwrap();
        let franchise = Franchise {
            id: 1,
            name: "Test Franchise".to_string(),
        };
        let result = FranchiseRepository { pool }
            .add_franchise(&franchise.name)
            .await
            .unwrap();
        assert_eq!(result, franchise.id);
    }

    #[async_std::test]
    async fn test_update_franchise() {
        let pool = get_memory_db_pool().await.unwrap();
        let franchise = Franchise {
            id: 1,
            name: "Test Franchise".to_string(),
        };
        query!(
            "INSERT INTO franchise (id, name) VALUES (?, ?)",
            franchise.id,
            franchise.name
        )
        .execute(&pool)
        .await
        .unwrap();

        let new_name = "New Test Franchise";
        let updated_franchise = Franchise {
            id: 1,
            name: new_name.to_string(),
        };

        let repository = FranchiseRepository { pool };
        let result = repository
            .update_franchise(&updated_franchise)
            .await
            .unwrap();
        assert_eq!(result, updated_franchise.id);

        let result = repository.get_all_franchises().await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], updated_franchise);
    }

    #[async_std::test]
    async fn test_delete_franchise() {
        let pool = get_memory_db_pool().await.unwrap();
        let franchise = Franchise {
            id: 1,
            name: "Test Franchise".to_string(),
        };
        query!(
            "INSERT INTO franchise (id, name) VALUES (?, ?)",
            franchise.id,
            franchise.name
        )
        .execute(&pool)
        .await
        .unwrap();

        let repository = FranchiseRepository { pool };

        let result = repository.delete_franchise(franchise.id).await.unwrap();
        assert_eq!(result, ());

        let result = repository.get_all_franchises().await.unwrap();
        assert_eq!(result.len(), 0);
    }

    #[async_std::test]
    async fn test_delete_franchise_in_use() {
        let pool = get_memory_db_pool().await.unwrap();
        let franchise = Franchise {
            id: 1,
            name: "Test Franchise".to_string(),
        };
        query!(
            "INSERT INTO franchise (id, name) VALUES (?, ?)",
            franchise.id,
            franchise.name
        )
        .execute(&pool)
        .await
        .unwrap();
        query!(
            "INSERT INTO software_title (id, name, franchise_id) VALUES (?, ?, ?)",
            1,
            "Test Software Title",
            franchise.id
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = FranchiseRepository { pool }
            .delete_franchise(franchise.id)
            .await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DatabaseError::InUse);
    }
}
