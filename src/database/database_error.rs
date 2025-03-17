use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database error: {0}")]
    SqlxError(#[from] SqlxError),

    #[error("Cannot delete because entity is in use")]
    InUse,
}
