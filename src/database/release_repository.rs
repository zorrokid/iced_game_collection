use std::sync::Arc;

use sqlx::{Pool, Sqlite};

use crate::model::model::Release;

use super::database_error::DatabaseError;

pub trait ReleaseReadRepository {
    async fn get_release(&self, id: i64) -> Result<Release, DatabaseError>;
    async fn get_releases_with_software_title(
        &self,
        software_title_id: i64,
    ) -> Result<Vec<Release>, DatabaseError>;
    async fn get_notes_for_release(&self, release_id: i64) -> Result<Vec<String>, DatabaseError>;
}

pub trait ReleaseWriteRepository {
    async fn add_release(&self, release_name: &str) -> Result<i64, DatabaseError>;
    async fn update_release(&self, release: &Release) -> Result<u64, DatabaseError>;
    async fn delete_release(&self, id: i64) -> Result<(), DatabaseError>;
    async fn add_note_for_release(&self, release_id: i64, note: &str) -> Result<(), DatabaseError>;
    async fn delete_note_for_release(
        &self,
        release_id: i64,
        note: &str,
    ) -> Result<(), DatabaseError>;
    async fn update_note_for_release(
        &self,
        release_id: i64,
        note: &str,
    ) -> Result<(), DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct ReleaseRepository {
    pool: Arc<Pool<Sqlite>>,
}

impl ReleaseRepository {
    pub fn new(pool: Arc<Pool<Sqlite>>) -> Self {
        Self { pool }
    }
}

impl ReleaseReadRepository for ReleaseRepository {
    async fn get_release(&self, id: i64) -> Result<Release, DatabaseError> {
        let release = sqlx::query_as!(Release, "SELECT id, name FROM release WHERE id = ?", id)
            .fetch_one(&*self.pool)
            .await?;
        Ok(release)
    }

    async fn get_releases_with_software_title(
        &self,
        software_title_id: i64,
    ) -> Result<Vec<Release>, DatabaseError> {
        let releases = sqlx::query_as!(
            Release,
            "SELECT r.id as id, r.name as name 
             FROM release r
             INNER JOIN release_software_title rst 
             ON r.id = rst.release_id
             WHERE rst.software_title_id = ?",
            software_title_id
        )
        .fetch_all(&*self.pool)
        .await?;

        Ok(releases)
    }

    async fn get_notes_for_release(&self, release_id: i64) -> Result<Vec<String>, DatabaseError> {
        let notes = sqlx::query!(
            "SELECT note FROM release_note WHERE release_id = ?",
            release_id
        )
        .fetch_all(&*self.pool)
        .await?;
        let notes = notes.into_iter().map(|row| row.note).collect();
        Ok(notes)
    }
}

impl ReleaseWriteRepository for ReleaseRepository {
    async fn add_release(&self, release_name: &str) -> Result<i64, DatabaseError> {
        let result = sqlx::query!("INSERT INTO release (name) VALUES (?)", release_name)
            .execute(&*self.pool)
            .await?;
        Ok(result.last_insert_rowid())
    }

    async fn update_release(&self, release: &Release) -> Result<u64, DatabaseError> {
        let result = sqlx::query!(
            "UPDATE release SET name = ? WHERE id = ?",
            release.name,
            release.id
        )
        .execute(&*self.pool)
        .await?;
        Ok(result.rows_affected())
    }

    async fn delete_release(&self, id: i64) -> Result<(), DatabaseError> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM release_collection_file WHERE release_id = ?",
            id
        )
        .fetch_one(&*self.pool)
        .await?;

        if count > 0 {
            return Err(DatabaseError::InUse);
        }
        sqlx::query!("DELETE FROM release WHERE id = ?", id)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }

    async fn add_note_for_release(&self, release_id: i64, note: &str) -> Result<(), DatabaseError> {
        sqlx::query!(
            "INSERT INTO release_note (release_id, note) VALUES (?, ?)",
            release_id,
            note
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    async fn delete_note_for_release(
        &self,
        release_id: i64,
        note: &str,
    ) -> Result<(), DatabaseError> {
        sqlx::query!(
            "DELETE FROM release_note WHERE release_id = ? AND note = ?",
            release_id,
            note
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    async fn update_note_for_release(
        &self,
        release_id: i64,
        note: &str,
    ) -> Result<(), DatabaseError> {
        sqlx::query!(
            "UPDATE release_note SET note = ? WHERE release_id = ?",
            note,
            release_id
        )
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
    async fn test_get_release() {
        let pool = get_memory_db_pool().await.unwrap();
        let release = Release {
            id: 1,
            name: "Test Release".to_string(),
        };
        query!(
            "INSERT INTO release (id, name) VALUES (?, ?)",
            release.id,
            release.name
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = ReleaseRepository { pool }
            .get_release(release.id)
            .await
            .unwrap();
        assert_eq!(result, release);
    }

    #[async_std::test]
    async fn test_get_releases_with_software_title() {
        let pool = get_memory_db_pool().await.unwrap();
        let release = Release {
            id: 1,
            name: "Test Release".to_string(),
        };
        query!(
            "INSERT INTO release (id, name) VALUES (?, ?)",
            release.id,
            release.name
        )
        .execute(&pool)
        .await
        .unwrap();

        query!(
            "INSERT INTO release_software_title (release_id, software_title_id) VALUES (?, ?)",
            release.id,
            1
        )
        .execute(&pool)
        .await
        .unwrap();

        let repository = ReleaseRepository { pool };

        let result = repository
            .get_releases_with_software_title(1)
            .await
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], release);
    }

    #[async_std::test]
    async fn test_get_notes_for_release() {
        let pool = get_memory_db_pool().await.unwrap();
        let release = Release {
            id: 1,
            name: "Test Release".to_string(),
        };
        query!(
            "INSERT INTO release (id, name) VALUES (?, ?)",
            release.id,
            release.name
        )
        .execute(&pool)
        .await
        .unwrap();

        query!(
            "INSERT INTO release_note (release_id, note) VALUES (?, ?)",
            release.id,
            "Test Note"
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = ReleaseRepository { pool }
            .get_notes_for_release(release.id)
            .await
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "Test Note");
    }

    #[async_std::test]
    async fn test_add_release() {
        let pool = get_memory_db_pool().await.unwrap();
        let release = Release {
            id: 1,
            name: "Test Release".to_string(),
        };
        let result = ReleaseRepository { pool }
            .add_release(&release.name)
            .await
            .unwrap();
        assert_eq!(result, release.id);
    }

    #[async_std::test]
    async fn test_update_release() {
        let pool = get_memory_db_pool().await.unwrap();
        let release = Release {
            id: 1,
            name: "Test Release".to_string(),
        };
        query!(
            "INSERT INTO release (id, name) VALUES (?, ?)",
            release.id,
            release.name
        )
        .execute(&pool)
        .await
        .unwrap();

        let new_name = "New Test Release";
        let updated_release = Release {
            id: 1,
            name: new_name.to_string(),
        };
        let repository = ReleaseRepository { pool };
        let result = repository.update_release(&updated_release).await.unwrap();
        assert_eq!(result, 1);

        let result = repository.get_release(release.id).await.unwrap();
        assert_eq!(result, updated_release);
    }

    #[async_std::test]
    async fn test_delete_release() {
        let pool = get_memory_db_pool().await.unwrap();
        let release = Release {
            id: 1,
            name: "Test Release".to_string(),
        };
        query!(
            "INSERT INTO release (id, name) VALUES (?, ?)",
            release.id,
            release.name
        )
        .execute(&pool)
        .await
        .unwrap();

        let repository = ReleaseRepository { pool };

        let result = repository.delete_release(release.id).await.unwrap();
        assert_eq!(result, ());

        let result = repository.get_release(release.id).await;
        assert!(result.is_err());
    }

    #[async_std::test]
    async fn test_delete_release_in_use() {
        let pool = get_memory_db_pool().await.unwrap();
        let release = Release {
            id: 1,
            name: "Test Release".to_string(),
        };
        query!(
            "INSERT INTO release (id, name) VALUES (?, ?)",
            release.id,
            release.name
        )
        .execute(&pool)
        .await
        .unwrap();

        query!(
            "INSERT INTO release_collection_file (release_id, collection_file_id) VALUES (?, ?)",
            release.id,
            1
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = ReleaseRepository { pool }.delete_release(release.id).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DatabaseError::InUse);
    }

    #[async_std::test]
    async fn test_add_note_for_release() {
        let pool = get_memory_db_pool().await.unwrap();
        let release = Release {
            id: 1,
            name: "Test Release".to_string(),
        };
        query!(
            "INSERT INTO release (id, name) VALUES (?, ?)",
            release.id,
            release.name
        )
        .execute(&pool)
        .await
        .unwrap();

        let note = "Test Note";
        let repository = ReleaseRepository { pool };
        repository
            .add_note_for_release(release.id, note)
            .await
            .unwrap();

        let result = repository.get_notes_for_release(release.id).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], note);
    }

    #[async_std::test]
    async fn test_delete_note_for_release() {
        let pool = get_memory_db_pool().await.unwrap();
        let release = Release {
            id: 1,
            name: "Test Release".to_string(),
        };
        query!(
            "INSERT INTO release (id, name) VALUES (?, ?)",
            release.id,
            release.name
        )
        .execute(&pool)
        .await
        .unwrap();

        let note = "Test Note";
        query!(
            "INSERT INTO release_note (release_id, note) VALUES (?, ?)",
            release.id,
            note
        )
        .execute(&pool)
        .await
        .unwrap();

        let repository = ReleaseRepository { pool };

        repository
            .delete_note_for_release(release.id, note)
            .await
            .unwrap();

        let result = repository.get_notes_for_release(release.id).await.unwrap();
        assert_eq!(result.len(), 0);
    }

    #[async_std::test]
    async fn test_update_note_for_release() {
        let pool = get_memory_db_pool().await.unwrap();
        let release = Release {
            id: 1,
            name: "Test Release".to_string(),
        };
        query!(
            "INSERT INTO release (id, name) VALUES (?, ?)",
            release.id,
            release.name
        )
        .execute(&pool)
        .await
        .unwrap();

        let note = "Test Note";
        query!(
            "INSERT INTO release_note (release_id, note) VALUES (?, ?)",
            release.id,
            note
        )
        .execute(&pool)
        .await
        .unwrap();

        let new_note = "New Test Note";
        let repository = ReleaseRepository { pool };
        repository
            .update_note_for_release(release.id, new_note)
            .await
            .unwrap();

        let result = repository.get_notes_for_release(release.id).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], new_note);
    }
}
