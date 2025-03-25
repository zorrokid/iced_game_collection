use std::sync::Arc;

use sqlx::{Pool, Sqlite};

use crate::model::model::SoftwareTitle;

use super::database_error::DatabaseError;

pub trait SoftwareTitleReadRepository {
    async fn get_software_title(&self, id: i64) -> Result<SoftwareTitle, DatabaseError>;
    //async fn get_software_titles(&self, ids: &Vec<i64>) -> Result<Vec<SoftwareTitle>, DatabaseError>;
    async fn get_all_software_titles(&self) -> Result<Vec<SoftwareTitle>, DatabaseError>;
    async fn is_software_title_in_release(
        &self,
        software_title_id: i64,
    ) -> Result<bool, DatabaseError>;
    //fn get_releases_by_game(&self, game_id: i64) -> Result<Option<ReleasesByGame>, Error>;
}

pub trait SoftwareTitleWriteRepository {
    async fn add_software_title(
        &self,
        name: &str,
        franchise_id: Option<i64>,
    ) -> Result<i64, DatabaseError>;
    async fn update_software_title(
        &self,
        software_title: &SoftwareTitle,
    ) -> Result<i64, DatabaseError>;
    async fn delete_software_title(&self, id: i64) -> Result<i64, DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct SoftwareTitleRepository {
    pool: Arc<Pool<Sqlite>>,
}

impl SoftwareTitleRepository {
    pub fn new(pool: Arc<Pool<Sqlite>>) -> Self {
        Self { pool }
    }
}

impl SoftwareTitleReadRepository for SoftwareTitleRepository {
    async fn get_software_title(&self, id: i64) -> Result<SoftwareTitle, DatabaseError> {
        let software_title = sqlx::query_as!(
            SoftwareTitle,
            "SELECT id, name, franchise_id FROM software_title WHERE id = ?",
            id
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(software_title)
    }

    async fn get_all_software_titles(&self) -> Result<Vec<SoftwareTitle>, DatabaseError> {
        let software_titles = sqlx::query_as!(
            SoftwareTitle,
            "SELECT id, name, franchise_id FROM software_title"
        )
        .fetch_all(&*self.pool)
        .await?;
        Ok(software_titles)
    }

    async fn is_software_title_in_release(
        &self,
        software_title_id: i64,
    ) -> Result<bool, DatabaseError> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM release_software_title WHERE software_title_id = ?",
            software_title_id
        )
        .fetch_one(&*self.pool)
        .await?;
        Ok(count > 0)
    }
}

impl SoftwareTitleWriteRepository for SoftwareTitleRepository {
    async fn add_software_title(
        &self,
        name: &str,
        franchise_id: Option<i64>,
    ) -> Result<i64, DatabaseError> {
        let result = sqlx::query!(
            "INSERT INTO software_title (name, franchise_id) VALUES (?, ?)",
            name,
            franchise_id
        )
        .execute(&*self.pool)
        .await?;
        Ok(result.last_insert_rowid())
    }

    async fn update_software_title(
        &self,
        software_title: &SoftwareTitle,
    ) -> Result<i64, DatabaseError> {
        let result = sqlx::query!(
            "UPDATE software_title SET name = ?, franchise_id = ? WHERE id = ?",
            software_title.name,
            software_title.franchise_id,
            software_title.id
        )
        .execute(&*self.pool)
        .await?;
        Ok(result.last_insert_rowid())
    }

    async fn delete_software_title(&self, id: i64) -> Result<i64, DatabaseError> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM release_software_title WHERE software_title_id = ?",
            id
        )
        .fetch_one(&*self.pool)
        .await?;
        if count > 0 {
            return Err(DatabaseError::InUse);
        }
        sqlx::query!("DELETE FROM software_title WHERE id = ?", id)
            .execute(&*self.pool)
            .await?;
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::{database_error::DatabaseError, database_with_sqlx::get_memory_db_pool};
    use sqlx::query;

    #[async_std::test]
    async fn test_get_software_title() {
        let pool = get_memory_db_pool().await.unwrap();
        let software_title = SoftwareTitle {
            id: 1,
            name: "Test Software Title".to_string(),
            franchise_id: None,
        };
        query!(
            "INSERT INTO software_title (id, name, franchise_id) VALUES (?, ?, ?)",
            software_title.id,
            software_title.name,
            software_title.franchise_id
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = SoftwareTitleRepository { pool }
            .get_software_title(software_title.id)
            .await
            .unwrap();
        assert_eq!(result, software_title);
    }

    #[async_std::test]
    async fn test_get_all_software_titles() {
        let pool = get_memory_db_pool().await.unwrap();
        let software_title = SoftwareTitle {
            id: 1,
            name: "Test Software Title".to_string(),
            franchise_id: None,
        };
        query!(
            "INSERT INTO software_title (id, name, franchise_id) VALUES (?, ?, ?)",
            software_title.id,
            software_title.name,
            software_title.franchise_id
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = SoftwareTitleRepository { pool }
            .get_all_software_titles()
            .await
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], software_title);
    }

    #[async_std::test]
    async fn test_is_software_title_in_release() {
        let pool = get_memory_db_pool().await.unwrap();
        query!(
            "INSERT INTO release (id, name) VALUES (?, ?)",
            1,
            "Test Release"
        )
        .execute(&pool)
        .await
        .unwrap();
        query!(
            "INSERT INTO software_title (id, name, franchise_id) VALUES (?, ?, ?)",
            1,
            "Test Software Title",
            None::<i64>
        )
        .execute(&pool)
        .await
        .unwrap();
        query!(
            "INSERT INTO release_software_title (release_id, software_title_id) VALUES (?, ?)",
            1,
            1
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = SoftwareTitleRepository { pool }
            .is_software_title_in_release(1)
            .await
            .unwrap();
        assert_eq!(result, true);
    }

    #[async_std::test]
    async fn test_add_software_title() {
        let pool = get_memory_db_pool().await.unwrap();
        let software_title = SoftwareTitle {
            id: 1,
            name: "Test Software Title".to_string(),
            franchise_id: None,
        };
        let result = SoftwareTitleRepository { pool }
            .add_software_title(&software_title)
            .await
            .unwrap();
        assert_eq!(result, software_title.id);
    }

    #[async_std::test]
    async fn test_update_software_title() {
        let pool = get_memory_db_pool().await.unwrap();
        let software_title = SoftwareTitle {
            id: 1,
            name: "Test Software Title".to_string(),
            franchise_id: None,
        };
        query!(
            "INSERT INTO software_title (id, name, franchise_id) VALUES (?, ?, ?)",
            software_title.id,
            software_title.name,
            software_title.franchise_id
        )
        .execute(&pool)
        .await
        .unwrap();

        let new_name = "New Test Software Title";
        let updated_software_title = SoftwareTitle {
            id: 1,
            name: new_name.to_string(),
            franchise_id: None,
        };
        let repository = SoftwareTitleRepository { pool };
        let result = repository
            .update_software_title(&updated_software_title)
            .await
            .unwrap();
        assert_eq!(result, updated_software_title.id);

        let result = repository
            .get_software_title(software_title.id)
            .await
            .unwrap();
        assert_eq!(result, updated_software_title);
    }

    #[async_std::test]
    async fn test_delete_software_title() {
        let pool = get_memory_db_pool().await.unwrap();
        let software_title = SoftwareTitle {
            id: 1,
            name: "Test Software Title".to_string(),
            franchise_id: None,
        };
        query!(
            "INSERT INTO software_title (id, name, franchise_id) VALUES (?, ?, ?)",
            software_title.id,
            software_title.name,
            software_title.franchise_id
        )
        .execute(&pool)
        .await
        .unwrap();

        let repository = SoftwareTitleRepository { pool };
        let result = repository
            .delete_software_title(software_title.id)
            .await
            .unwrap();
        assert_eq!(result, ());

        let result = repository.get_software_title(software_title.id).await;
        assert!(result.is_err());
    }

    #[async_std::test]
    async fn test_delete_software_title_in_use() {
        let pool = get_memory_db_pool().await.unwrap();
        query!(
            "INSERT INTO release (id, name) VALUES (?, ?)",
            1,
            "Test Release"
        )
        .execute(&pool)
        .await
        .unwrap();
        query!(
            "INSERT INTO software_title (id, name, franchise_id) VALUES (?, ?, ?)",
            1,
            "Test Software Title",
            None::<i64>
        )
        .execute(&pool)
        .await
        .unwrap();
        query!(
            "INSERT INTO release_software_title (release_id, software_title_id) VALUES (?, ?)",
            1,
            1
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = SoftwareTitleRepository { pool }
            .delete_software_title(1)
            .await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DatabaseError::InUse);
    }
}
