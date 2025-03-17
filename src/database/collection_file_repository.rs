use std::sync::Arc;

use sqlx::{sqlite::SqliteRow, FromRow, Pool, Row, Sqlite};

use crate::model::collection_file::{ArchiveType, CollectionFile, CollectionFileType};

use super::database_error::DatabaseError;

pub trait CollectionFileReadRepository {
    async fn get_collection_files_for_release(
        &self,
        release_id: i64,
    ) -> Result<Vec<CollectionFile>, DatabaseError>;

    async fn is_collection_file_in_release(
        &self,
        collection_file_id: i64,
    ) -> Result<bool, DatabaseError>;
}

pub trait CollectionFileWriteRepository {
    async fn add_collection_file(
        &self,
        original_file_name: String,
        is_archive: bool,
        archive_type: ArchiveType,
        file_type: CollectionFileType,
    ) -> Result<i64, DatabaseError>;
    async fn delete_collection_file(&self, id: i64) -> Result<(), DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct CollectionFileRepository {
    pool: Arc<Pool<Sqlite>>,
}

impl CollectionFileRepository {
    pub fn new(pool: Arc<Pool<Sqlite>>) -> Self {
        Self { pool }
    }
}

impl FromRow<'_, SqliteRow> for CollectionFile {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        // TODO - handle try_into error instead of unwrap
        let archive_type: ArchiveType = row.try_get::<i64, _>("archive_type")?.try_into().unwrap();
        let file_type: CollectionFileType = row.try_get::<i64, _>("file_type")?.try_into().unwrap();
        Ok(Self {
            id: row.try_get("id")?,
            original_file_name: row.try_get("original_file_name")?,
            is_archive: row.try_get::<i64, _>("is_archive")? == 1,
            archive_type,
            file_type,
        })
    }
}

impl CollectionFileReadRepository for CollectionFileRepository {
    async fn get_collection_files_for_release(
        &self,
        release_id: i64,
    ) -> Result<Vec<CollectionFile>, DatabaseError> {
        let collection_files = sqlx::query_as(
            "SELECT c.id, c.original_file_name, c.is_archive, c.archive_type, c.file_type 
             FROM collection_file c 
             INNER JOIN release_collection_file rcf
             ON c.id = rcf.collection_file_id
             WHERE rcf.release_id = ?",
        )
        .bind(release_id)
        .fetch_all(&*self.pool)
        .await?;
        Ok(collection_files)
    }

    async fn is_collection_file_in_release(
        &self,
        collection_file_id: i64,
    ) -> Result<bool, DatabaseError> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) 
             FROM release_collection_file 
             WHERE collection_file_id = ?",
            collection_file_id
        )
        .fetch_one(&*self.pool)
        .await?;
        Ok(count > 0)
    }
}

impl CollectionFileWriteRepository for CollectionFileRepository {
    async fn add_collection_file(
        &self,
        original_file_name: String,
        is_archive: bool,
        archive_type: ArchiveType,
        file_type: CollectionFileType,
    ) -> Result<i64, DatabaseError> {
        let archive_type = archive_type as i64;
        let file_type = file_type as i64;
        let result = sqlx::query!(
            "INSERT INTO collection_file (
                original_file_name, 
                is_archive, 
                archive_type, 
                file_type) 
             VALUES (?, ?, ?, ?)",
            original_file_name,
            is_archive,
            archive_type,
            file_type
        )
        .execute(&self.pool)
        .await?;
        Ok(result.last_insert_rowid())
    }

    async fn delete_collection_file(&self, id: i64) -> Result<(), DatabaseError> {
        let is_in_use = sqlx::query_scalar!(
            "SELECT COUNT(*) 
             FROM release_collection_file 
             WHERE collection_file_id = ?",
            id
        )
        .fetch_one(&self.pool)
        .await?;

        if is_in_use > 0 {
            return Err(DatabaseError::InUse);
        }

        sqlx::query!("DELETE FROM collection_file WHERE id = ?", id)
            .execute(&self.pool)
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
    async fn test_get_collection_files_for_release() {
        let pool = get_memory_db_pool().await.unwrap();
        let collection_file = CollectionFile {
            id: 1,
            original_file_name: "test.zip".to_string(),
            is_archive: true,
            archive_type: None,
            file_type: None,
        };
        query!(
            "INSERT INTO collection_file (id, original_file_name, is_archive, archive_type, file_type) VALUES (?, ?, ?, ?, ?)",
            collection_file.id,
            collection_file.original_file_name,
            collection_file.is_archive,
            collection_file.archive_type,
            collection_file.file_type
        )
        .execute(&pool)
        .await
        .unwrap();
        query!(
            "INSERT INTO release_collection_file (release_id, collection_file_id) VALUES (?, ?)",
            1,
            1
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = CollectionFileRepository { pool }
            .get_collection_files_for_release(1)
            .await
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], collection_file);
    }

    #[async_std::test]
    async fn test_is_collection_file_in_release() {
        let pool = get_memory_db_pool().await.unwrap();
        query!(
            "INSERT INTO collection_file (id, original_file_name, is_archive, archive_type, file_type) VALUES (?, ?, ?, ?, ?)",
            1,
            "test.zip",
            true,
            None::<String>,
            None::<String>
        )
        .execute(&pool)
        .await
        .unwrap();
        query!(
            "INSERT INTO release_collection_file (release_id, collection_file_id) VALUES (?, ?)",
            1,
            1
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = CollectionFileRepository { pool }
            .is_collection_file_in_release(1)
            .await
            .unwrap();
        assert_eq!(result, true);
    }

    #[async_std::test]
    async fn test_add_collection_file() {
        let pool = get_memory_db_pool().await.unwrap();
        let collection_file = CollectionFile {
            id: 1,
            original_file_name: "test.zip".to_string(),
            is_archive: true,
            archive_type: None,
            file_type: None,
        };
        let result = CollectionFileRepository { pool }
            .add_collection_file(
                collection_file.original_file_name,
                collection_file.is_archive,
                collection_file.archive_type.unwrap(),
                collection_file.file_type.unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(result, collection_file.id);
    }

    #[async_std::test]
    async fn test_delete_collection_file() {
        let pool = get_memory_db_pool().await.unwrap();
        let collection_file = CollectionFile {
            id: 1,
            original_file_name: "test.zip".to_string(),
            is_archive: true,
            archive_type: None,
            file_type: None,
        };
        query!(
            "INSERT INTO collection_file (id, original_file_name, is_archive, archive_type, file_type) VALUES (?, ?, ?, ?, ?)",
            collection_file.id,
            collection_file.original_file_name,
            collection_file.is_archive,
            collection_file.archive_type,
            collection_file.file_type
        )
        .execute(&pool)
        .await
        .unwrap();

        let repository = CollectionFileRepository { pool };

        let result = repository
            .delete_collection_file(collection_file.id)
            .await
            .unwrap();
        assert_eq!(result, ());

        let result = repository
            .get_collection_files_for_release(1)
            .await
            .unwrap();
        assert_eq!(result.len(), 0);
    }

    #[async_std::test]
    async fn test_delete_collection_file_in_use() {
        let pool = get_memory_db_pool().await.unwrap();
        let collection_file = CollectionFile {
            id: 1,
            original_file_name: "test.zip".to_string(),
            is_archive: true,
            archive_type: None,
            file_type: None,
        };
        query!(
            "INSERT INTO collection_file (id, original_file_name, is_archive, archive_type, file_type) VALUES (?, ?, ?, ?, ?)",
            collection_file.id,
            collection_file.original_file_name,
            collection_file.is_archive,
            collection_file.archive_type,
            collection_file.file_type
        )
        .execute(&pool)
        .await
        .unwrap();
        query!(
            "INSERT INTO release_collection_file (release_id, collection_file_id) VALUES (?, ?)",
            1,
            1
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = CollectionFileRepository { pool }
            .delete_collection_file(collection_file.id)
            .await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DatabaseError::InUse);
    }
}
