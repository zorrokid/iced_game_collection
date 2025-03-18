use std::sync::Arc;

use sqlx::{Pool, Sqlite};

use crate::model::model::System;

use super::database_error::DatabaseError;

pub trait SystemReadRepository {
    async fn get_system(&self, id: i64) -> Result<System, DatabaseError>;
    async fn get_systems(&self) -> Result<Vec<System>, DatabaseError>;
    async fn is_system_in_use(&self, system_id: i64) -> Result<bool, DatabaseError>;
    async fn get_notes_for_system(&self, system_id: i64) -> Result<Vec<String>, DatabaseError>;
}

pub trait SystemWriteRepository {
    async fn add_system(&self, name: &String) -> Result<i64, DatabaseError>;
    async fn update_system(&self, system: &System) -> Result<i64, DatabaseError>;
    async fn delete_system(&self, id: i64) -> Result<(), DatabaseError>;
    async fn add_note_for_system(&self, system_id: i64, note: &str) -> Result<(), DatabaseError>;
    async fn update_note_for_system(
        &self,
        system_id: i64,
        note_id: i64,
        note: &str,
    ) -> Result<(), DatabaseError>;
    async fn delete_note_for_system(&self, system_id: i64, note: &str)
        -> Result<(), DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct SystemRepository {
    pool: Arc<Pool<Sqlite>>,
}

impl SystemRepository {
    pub fn new(pool: Arc<Pool<Sqlite>>) -> Self {
        Self { pool }
    }
}

impl SystemReadRepository for SystemRepository {
    async fn get_system(&self, id: i64) -> Result<System, DatabaseError> {
        let system = sqlx::query_as!(
            System,
            "SELECT id, name 
             FROM system WHERE id = ?",
            id
        )
        .fetch_one(&*self.pool)
        .await?;
        Ok(system)
    }

    async fn get_systems(&self) -> Result<Vec<System>, DatabaseError> {
        let systems = sqlx::query_as!(System, "SELECT id, name FROM system")
            .fetch_all(&*self.pool)
            .await?;
        Ok(systems)
    }

    async fn is_system_in_use(&self, system_id: i64) -> Result<bool, DatabaseError> {
        let releases_count = sqlx::query_scalar!(
            "SELECT COUNT(*) 
             FROM release_system 
             WHERE system_id = ?",
            system_id
        )
        .fetch_one(&*self.pool)
        .await?;

        let emulators_count = sqlx::query_scalar!(
            "SELECT COUNT(*) 
             FROM emulator 
             WHERE system_id = ?",
            system_id
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(releases_count > 0 || emulators_count > 0)
    }

    async fn get_notes_for_system(&self, system_id: i64) -> Result<Vec<String>, DatabaseError> {
        let notes = sqlx::query!(
            "SELECT note FROM system_note WHERE system_id = ?",
            system_id
        )
        .fetch_all(&*self.pool)
        .await?;
        let notes = notes.into_iter().map(|row| row.note).collect();
        Ok(notes)
    }
}

impl SystemWriteRepository for SystemRepository {
    async fn add_system(&self, name: &String) -> Result<i64, DatabaseError> {
        let result = sqlx::query!("INSERT INTO system (name) VALUES (?)", name)
            .execute(&*self.pool)
            .await?;
        Ok(result.last_insert_rowid())
    }

    async fn update_system(&self, system: &System) -> Result<i64, DatabaseError> {
        let result = sqlx::query!(
            "UPDATE system SET name = ? WHERE id = ?",
            system.name,
            system.id
        )
        .execute(&*self.pool)
        .await?;
        Ok(result.last_insert_rowid())
    }

    async fn delete_system(&self, id: i64) -> Result<(), DatabaseError> {
        let is_in_use_in_releases = sqlx::query_scalar!(
            "SELECT COUNT(*) 
             FROM release_system 
             WHERE system_id = ?",
            id
        )
        .fetch_one(&*self.pool)
        .await?;

        let is_in_use_in_emulators = sqlx::query_scalar!(
            "SELECT COUNT(*) 
             FROM emulator 
             WHERE system_id = ?",
            id
        )
        .fetch_one(&*self.pool)
        .await?;

        if is_in_use_in_releases > 0 || is_in_use_in_emulators > 0 {
            return Err(DatabaseError::InUse);
        }

        sqlx::query!("DELETE FROM system WHERE id = ?", id)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }

    async fn add_note_for_system(&self, system_id: i64, note: &str) -> Result<(), DatabaseError> {
        sqlx::query!(
            "INSERT INTO system_note (system_id, note) VALUES (?, ?)",
            system_id,
            note
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    async fn update_note_for_system(
        &self,
        system_id: i64,
        note_id: i64,
        note: &str,
    ) -> Result<(), DatabaseError> {
        sqlx::query!(
            "UPDATE system_note SET note = ? WHERE system_id = ? AND id = ?",
            note,
            system_id,
            note_id
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    async fn delete_note_for_system(
        &self,
        system_id: i64,
        note: &str,
    ) -> Result<(), DatabaseError> {
        sqlx::query!(
            "DELETE FROM system_note WHERE system_id = ? AND note = ?",
            system_id,
            note
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::{database_error::DatabaseError, database_with_sqlx::get_memory_db_pool};
    use sqlx::query;

    #[async_std::test]
    async fn test_get_system() {
        let pool = get_memory_db_pool().await.unwrap();
        let system = System {
            id: 1,
            name: "Test System".to_string(),
        };
        query!(
            "INSERT INTO system (id, name) VALUES (?, ?)",
            system.id,
            system.name
        )
        .execute(&pool)
        .await
        .unwrap();

        let repository = SystemRepository { pool };

        let result = repository.get_system(system.id).await.unwrap();
        assert_eq!(result, system);
    }

    #[async_std::test]
    async fn test_get_systems() {
        let pool = get_memory_db_pool().await.unwrap();
        let system = System {
            id: 1,
            name: "Test System".to_string(),
        };
        query!(
            "INSERT INTO system (id, name) VALUES (?, ?)",
            system.id,
            system.name
        )
        .execute(&pool)
        .await
        .unwrap();

        let repository = SystemRepository { pool };
        let result = repository.get_systems().await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], system);
    }

    #[async_std::test]
    async fn test_is_system_in_release() {
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
            "INSERT INTO system (id, name) VALUES (?, ?)",
            1,
            "Test System"
        )
        .execute(&pool)
        .await
        .unwrap();
        query!(
            "INSERT INTO release_system (release_id, system_id) VALUES (?, ?)",
            1,
            1
        )
        .execute(&pool)
        .await
        .unwrap();

        let repository = SystemRepository { pool };
        let result = repository.is_system_in_release(1).await.unwrap();
        assert_eq!(result, true);
    }

    #[async_std::test]
    async fn test_get_notes_for_system() {
        let pool = get_memory_db_pool().await.unwrap();
        query!(
            "INSERT INTO system (id, name) VALUES (?, ?)",
            1,
            "Test System"
        )
        .execute(&pool)
        .await
        .unwrap();
        query!(
            "INSERT INTO system_note (system_id, note) VALUES (?, ?)",
            1,
            "Test Note"
        )
        .execute(&pool)
        .await
        .unwrap();

        let repository = SystemRepository { pool };
        let result = repository.get_notes_for_system(1).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "Test Note");
    }

    #[async_std::test]
    async fn test_add_system() {
        let pool = get_memory_db_pool().await.unwrap();
        let system = System {
            id: 1,
            name: "Test System".to_string(),
        };
        let repository = SystemRepository { pool };
        let result = repository.add_system(&system.name).await.unwrap();
        assert_eq!(result, system.id);
    }

    #[async_std::test]
    async fn test_update_system() {
        let pool = get_memory_db_pool().await.unwrap();
        let system = System {
            id: 1,
            name: "Test System".to_string(),
        };
        query!(
            "INSERT INTO system (id, name) VALUES (?, ?)",
            system.id,
            system.name
        )
        .execute(&pool)
        .await
        .unwrap();

        let new_name = "New Test System";
        let updated_system = System {
            id: 1,
            name: new_name.to_string(),
        };
        let repository = SystemRepository { pool };
        let result = repository.update_system(&updated_system).await.unwrap();
        assert_eq!(result, updated_system.id);

        let result = repository.get_system(system.id).await.unwrap();
        assert_eq!(result, updated_system);
    }

    #[async_std::test]
    async fn test_delete_system() {
        let pool = get_memory_db_pool().await.unwrap();
        let system = System {
            id: 1,
            name: "Test System".to_string(),
        };
        query!(
            "INSERT INTO system (id, name) VALUES (?, ?)",
            system.id,
            system.name
        )
        .execute(&pool)
        .await
        .unwrap();

        let repository = SystemRepository { pool };
        let result = repository.delete_system(system.id).await.unwrap();
        assert_eq!(result, ());

        let result = repository.get_system(system.id).await;
        assert!(result.is_err());
    }

    #[async_std::test]
    async fn test_delete_system_in_use_with_emulators() {
        let pool = get_memory_db_pool().await.unwrap();
        query!(
            "INSERT INTO system (id, name) VALUES (?, ?)",
            1,
            "Test System"
        )
        .execute(&pool)
        .await
        .unwrap();
        query!(
            "INSERT INTO emulator (id, name, system_id) VALUES (?, ?, ?)",
            1,
            "Test Emulator",
            1
        )
        .execute(&pool)
        .await
        .unwrap();

        let repository = SystemRepository { pool };
        let result = repository.delete_system(1).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DatabaseError::InUse);
    }

    #[async_std::test]
    async fn test_delete_system_in_use_with_releases() {
        let pool = get_memory_db_pool().await.unwrap();
        query!(
            "INSERT INTO system (id, name) VALUES (?, ?)",
            1,
            "Test System"
        )
        .execute(&pool)
        .await
        .unwrap();

        query!(
            "INSERT INTO release (id, name) VALUES (?, ?)",
            1,
            "Test Release",
        )
        .execute(&pool)
        .await
        .unwrap();

        query!(
            "INSERT INTO release_system (release_id, system_id) VALUES (?, ?)",
            1,
            1
        )
        .execute(&pool)
        .await
        .unwrap();

        let repository = SystemRepository { pool };
        let result = repository.delete_system(1).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DatabaseError::InUse);
    }

    #[async_std::test]
    async fn test_add_note_for_system() {
        let pool = get_memory_db_pool().await.unwrap();
        query!(
            "INSERT INTO system (id, name) VALUES (?, ?)",
            1,
            "Test System"
        )
        .execute(&pool)
        .await
        .unwrap();

        let note = "Test Note";
        let repository = SystemRepository { pool };
        repository.add_note_for_system(1, note).await.unwrap();

        let result = repository.get_notes_for_system(1).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], note);
    }

    #[async_std::test]
    async fn test_update_note_for_system() {
        let pool = get_memory_db_pool().await.unwrap();
        query!(
            "INSERT INTO system (id, name) VALUES (?, ?)",
            1,
            "Test System"
        )
        .execute(&pool)
        .await
        .unwrap();
        query!(
            "INSERT INTO system_note (system_id, note) VALUES (?, ?)",
            1,
            "Test Note"
        )
        .execute(&pool)
        .await
        .unwrap();

        let new_note = "New Test Note";
        let repository = SystemRepository { pool };
        repository
            .update_note_for_system(1, 1, new_note)
            .await
            .unwrap();

        let result = repository.get_notes_for_system(1).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], new_note);
    }

    #[async_std::test]
    async fn test_delete_note_for_system() {
        let pool = get_memory_db_pool().await.unwrap();
        query!(
            "INSERT INTO system (id, name) VALUES (?, ?)",
            1,
            "Test System"
        )
        .execute(&pool)
        .await
        .unwrap();
        query!(
            "INSERT INTO system_note (system_id, note) VALUES (?, ?)",
            1,
            "Test Note"
        )
        .execute(&pool)
        .await
        .unwrap();

        let repository = SystemRepository { pool };
        repository
            .delete_note_for_system(1, "Test Note")
            .await
            .unwrap();

        let result = repository.get_notes_for_system(1).await.unwrap();
        assert_eq!(result.len(), 0);
    }
}
