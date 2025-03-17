use std::sync::Arc;

use crate::model::model::Emulator;

pub trait EmulatorReadRepository {
    async fn get_emulators(&self) -> Result<Vec<Emulator>, DatabaseError>;
    async fn get_emulator(&self, id: i64) -> Result<Emulator, DatabaseError>;
    async fn get_notes_for_emulator(&self, emulator_id: i64) -> Result<Vec<String>, DatabaseError>;
}

pub trait EmulatorWriteRepository {
    async fn add_emulator(
        &self,
        name: &String,
        executable: &String,
        arguments: &String,
        system_id: i64,
        extract_files: bool,
        supported_extensions: &String,
    ) -> Result<i64, DatabaseError>;
    async fn delete_emulator(&self, id: i64) -> Result<(), DatabaseError>;
    async fn update_emulator(&self, emulator: &Emulator) -> Result<i64, DatabaseError>;
    async fn add_note_for_emulator(
        &self,
        emulator_id: i64,
        note: &str,
    ) -> Result<(), DatabaseError>;
    async fn delete_note_for_emulator(
        &self,
        emulator_id: i64,
        note: &str,
    ) -> Result<(), DatabaseError>;
    async fn update_note_for_emulator(
        &self,
        emulator_id: i64,
        note: &str,
    ) -> Result<(), DatabaseError>;
}

#[derive(Debug, Clone)]
pub struct EmulatorRepository {
    pool: Arc<Pool<Sqlite>>,
}

impl EmulatorRepository {
    pub fn new(pool: Arc<Pool<Sqlite>>) -> Self {
        Self { pool }
    }
}

use sqlx::sqlite::SqliteRow;
use sqlx::{FromRow, Pool, Row, Sqlite};

use super::database_error::DatabaseError;

impl FromRow<'_, SqliteRow> for Emulator {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            executable: row.try_get("executable")?,
            arguments: row.try_get("arguments")?,
            system_id: row.try_get("system_id")?,
            extract_files: row.try_get::<i64, _>("extract_files")? != 0, // Convert i64 to bool
            supported_extensions: row.try_get("supported_extensions")?,
        })
    }
}

impl EmulatorReadRepository for EmulatorRepository {
    async fn get_emulators(&self) -> Result<Vec<Emulator>, DatabaseError> {
        let emulators = sqlx::query_as::<_, Emulator>(
            "SELECT id, name, executable, arguments, system_id, 
             extract_files as extract_files: bool, supported_extensions 
             FROM emulator
            ",
        )
        .fetch_all(&*self.pool)
        .await?;
        Ok(emulators)
    }

    async fn get_emulator(&self, id: i64) -> Result<Emulator, DatabaseError> {
        let emulator = sqlx::query_as::<_, Emulator>(
            "SELECT id, name, executable, arguments, system_id, 
             extract_files as extract_files: bool, supported_extensions 
             FROM emulator 
             WHERE id = ?",
        )
        .bind(id)
        .fetch_one(&*self.pool)
        .await?;
        Ok(emulator)
    }

    async fn get_notes_for_emulator(&self, emulator_id: i64) -> Result<Vec<String>, DatabaseError> {
        let notes = sqlx::query!(
            "SELECT note FROM emulator_note WHERE emulator_id = ?",
            emulator_id
        )
        .fetch_all(&*self.pool)
        .await?;
        let notes = notes.into_iter().map(|row| row.note).collect();
        Ok(notes)
    }
}

impl EmulatorWriteRepository for EmulatorRepository {
    async fn add_emulator(
        &self,
        name: &String,
        executable: &String,
        arguments: &String,
        system_id: i64,
        extract_files: bool,
        supported_extensions: &String,
    ) -> Result<i64, DatabaseError> {
        let result = sqlx::query!(
            "INSERT INTO emulator (
                name, 
                executable, 
                arguments, 
                system_id, 
                extract_files, 
                supported_extensions
            ) 
            VALUES (?, ?, ?, ?, ?, ?)",
            name,
            executable,
            arguments,
            system_id,
            extract_files,
            supported_extensions
        )
        .execute(&*self.pool)
        .await?;
        Ok(result.last_insert_rowid())
    }

    async fn delete_emulator(&self, id: i64) -> Result<(), DatabaseError> {
        sqlx::query!("DELETE FROM emulator WHERE id = ?", id)
            .execute(&*self.pool)
            .await?;
        Ok(())
    }

    async fn update_emulator(&self, emulator: &Emulator) -> Result<i64, DatabaseError> {
        let result = sqlx::query!(
            "UPDATE emulator SET 
             name = ?, 
             executable = ?, 
             arguments = ?, 
             system_id = ?, 
             extract_files = ?, 
             supported_extensions = ? 
             WHERE id = ?",
            emulator.name,
            emulator.executable,
            emulator.arguments,
            emulator.system_id,
            emulator.extract_files,
            emulator.supported_extensions,
            emulator.id
        )
        .execute(&*self.pool)
        .await?;
        Ok(result.last_insert_rowid())
    }

    async fn add_note_for_emulator(
        &self,
        emulator_id: i64,
        note: &str,
    ) -> Result<(), DatabaseError> {
        sqlx::query!(
            "INSERT INTO emulator_note (emulator_id, note) VALUES (?, ?)",
            emulator_id,
            note
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    async fn delete_note_for_emulator(
        &self,
        emulator_id: i64,
        note: &str,
    ) -> Result<(), DatabaseError> {
        sqlx::query!(
            "DELETE FROM emulator_note WHERE emulator_id = ? AND note = ?",
            emulator_id,
            note
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    async fn update_note_for_emulator(
        &self,
        emulator_id: i64,
        note: &str,
    ) -> Result<(), DatabaseError> {
        sqlx::query!(
            "UPDATE emulator_note SET note = ? WHERE emulator_id = ?",
            note,
            emulator_id
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
    async fn test_get_emulators() {
        let pool = get_memory_db_pool().await;
        let emulator = Emulator {
            id: 1,
            name: "Test Emulator".to_string(),
            executable: "test.exe".to_string(),
            arguments: "test".to_string(),
            system_id: 1,
            extract_files: false,
            supported_extensions: "zip".to_string(),
        };
        query!(
            "INSERT INTO emulator (id, name, executable, arguments, system_id, extract_files, supported_extensions) VALUES (?, ?, ?, ?, ?, ?, ?)",
            emulator.id,
            emulator.name,
            emulator.executable,
            emulator.arguments,
            emulator.system_id,
            emulator.extract_files,
            emulator.supported_extensions
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = EmulatorRepository { pool }.get_emulators().await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], emulator);
    }

    #[async_std::test]
    async fn test_get_emulator() {
        let pool = get_memory_db_pool().await;
        let emulator = Emulator {
            id: 1,
            name: "Test Emulator".to_string(),
            executable: "test.exe".to_string(),
            arguments: "test".to_string(),
            system_id: 1,
            extract_files: false,
            supported_extensions: "zip".to_string(),
        };
        query!(
            "INSERT INTO emulator (id, name, executable, arguments, system_id, extract_files, supported_extensions) VALUES (?, ?, ?, ?, ?, ?, ?)",
            emulator.id,
            emulator.name,
            emulator.executable,
            emulator.arguments,
            emulator.system_id,
            emulator.extract_files,
            emulator.supported_extensions
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = EmulatorRepository { pool }
            .get_emulator(emulator.id)
            .await
            .unwrap();
        assert_eq!(result, emulator);
    }

    #[async_std::test]
    async fn test_get_notes_for_emulator() {
        let pool = get_memory_db_pool().await;
        query!(
            "INSERT INTO emulator (id, name, executable, arguments, system_id, extract_files, supported_extensions) VALUES (?, ?, ?, ?, ?, ?, ?)",
            1,
            "Test Emulator",
            "test.exe",
            "test",
            1,
            false,
            "zip"
        )
        .execute(&pool)
        .await
        .unwrap();
        query!(
            "INSERT INTO emulator_note (emulator_id, note) VALUES (?, ?)",
            1,
            "Test Note"
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = EmulatorRepository { pool }
            .get_notes_for_emulator(1)
            .await
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "Test Note");
    }

    #[async_std::test]
    async fn test_add_emulator() {
        let pool = get_memory_db_pool().await;
        let emulator = Emulator {
            id: 1,
            name: "Test Emulator".to_string(),
            executable: "test.exe".to_string(),
            arguments: "test".to_string(),
            system_id: 1,
            extract_files: false,
            supported_extensions: "zip".to_string(),
        };
        let result = EmulatorRepository { pool }
            .add_emulator(
                &emulator.name,
                &emulator.executable,
                &emulator.arguments,
                emulator.system_id,
                emulator.extract_files,
                &emulator.supported_extensions,
            )
            .await
            .unwrap();
        assert_eq!(result, emulator.id);
    }

    #[async_std::test]
    async fn test_delete_emulator() {
        let pool = get_memory_db_pool().await;
        let emulator = Emulator {
            id: 1,
            name: "Test Emulator".to_string(),
            executable: "test.exe".to_string(),
            arguments: "test".to_string(),
            system_id: 1,
            extract_files: false,
            supported_extensions: "zip".to_string(),
        };
        query!(
            "INSERT INTO emulator (id, name, executable, arguments, system_id, extract_files, supported_extensions) VALUES (?, ?, ?, ?, ?, ?, ?)",
            emulator.id,
            emulator.name,
            emulator.executable,
            emulator.arguments,
            emulator.system_id,
            emulator.extract_files,
            emulator.supported_extensions
        )
        .execute(&pool)
        .await
        .unwrap();

        let repository = EmulatorRepository { pool };

        let result = repository.delete_emulator(emulator.id).await.unwrap();
        assert_eq!(result, ());

        let result = repository.get_emulator(emulator.id).await;
        assert!(result.is_err());
    }

    #[async_std::test]
    async fn test_update_emulator() {
        let pool = get_memory_db_pool().await;
        let emulator = Emulator {
            id: 1,
            name: "Test Emulator".to_string(),
            executable: "test.exe".to_string(),
            arguments: "test".to_string(),
            system_id: 1,
            extract_files: false,
            supported_extensions: "zip".to_string(),
        };
        query!(
            "INSERT INTO emulator (id, name, executable, arguments, system_id, extract_files, supported_extensions) VALUES (?, ?, ?, ?, ?, ?, ?)",
            emulator.id,
            emulator.name,
            emulator.executable,
            emulator.arguments,
            emulator.system_id,
            emulator.extract_files,
            emulator.supported_extensions
        )
        .execute(&pool)
        .await
        .unwrap();

        let new_name = "New Test Emulator";
        let updated_emulator = Emulator {
            id: 1,
            name: new_name.to_string(),
            executable: "test.exe".to_string(),
            arguments: "test".to_string(),
            system_id: 1,
            extract_files: false,
            supported_extensions: "zip".to_string(),
        };

        let repository = EmulatorRepository { pool };
        let result = repository.update_emulator(&updated_emulator).await.unwrap();
        assert_eq!(result, updated_emulator.id);

        let result = repository.get_emulator(emulator.id).await.unwrap();
        assert_eq!(result, updated_emulator);
    }

    #[async_std::test]
    async fn test_add_note_for_emulator() {
        let pool = get_memory_db_pool().await;
        query!(
            "INSERT INTO emulator (id, name, executable, arguments, system_id, extract_files, supported_extensions) VALUES (?, ?, ?, ?, ?, ?, ?)",
            1,
            "Test Emulator",
            "test.exe",
            "test",
            1,
            false,
            "zip"
        )
        .execute(&pool)
        .await
        .unwrap();

        let repository = EmulatorRepository { pool };
        let note = "Test Note";
        repository.add_note_for_emulator(1, note).await.unwrap();

        let result = repository.get_notes_for_emulator(1).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], note);
    }

    #[async_std::test]
    async fn test_delete_note_for_emulator() {
        let pool = get_memory_db_pool().await;
        query!(
            "INSERT INTO emulator (id, name, executable, arguments, system_id, extract_files, supported_extensions) VALUES (?, ?, ?, ?, ?, ?, ?)",
            1,
            "Test Emulator",
            "test.exe",
            "test",
            1,
            false,
            "zip"
        )
        .execute(&pool)
        .await
        .unwrap();
        query!(
            "INSERT INTO emulator_note (emulator_id, note) VALUES (?, ?)",
            1,
            "Test Note"
        )
        .execute(&pool)
        .await
        .unwrap();

        let repository = EmulatorRepository { pool };

        repository
            .delete_note_for_emulator(1, "Test Note")
            .await
            .unwrap();

        let result = repository.get_notes_for_emulator(1).await.unwrap();
        assert_eq!(result.len(), 0);
    }

    #[async_std::test]
    async fn test_update_note_for_emulator() {
        let pool = get_memory_db_pool().await;
        query!(
            "INSERT INTO emulator (id, name, executable, arguments, system_id, extract_files, supported_extensions) VALUES (?, ?, ?, ?, ?, ?, ?)",
            1,
            "Test Emulator",
            "test.exe",
            "test",
            1,
            false,
            "zip"
        )
        .execute(&pool)
        .await
        .unwrap();
        query!(
            "INSERT INTO emulator_note (emulator_id, note) VALUES (?, ?)",
            1,
            "Test Note"
        )
        .execute(&pool)
        .await
        .unwrap();

        let repository = EmulatorRepository { pool };
        let new_note = "New Test Note";
        repository
            .update_note_for_emulator(1, new_note)
            .await
            .unwrap();

        let result = repository.get_notes_for_emulator(1).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], new_note);
    }
}
