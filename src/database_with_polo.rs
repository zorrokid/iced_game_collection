use lazy_static::lazy_static;
use polodb_core::{bson::doc, CollectionT, Database};

use crate::{error::Error, model::model::System};

const COLLECTION_DATABASE_NAME: &str = "iced_game_collection.db";

pub struct DatabaseWithPolo {
    db: Database,
}

impl DatabaseWithPolo {
    pub fn new() -> Self {
        let db = Database::open_path(COLLECTION_DATABASE_NAME).unwrap();
        Self { db }
    }

    pub fn get_instance() -> &'static Self {
        lazy_static! {
            static ref INSTANCE: DatabaseWithPolo = DatabaseWithPolo::new();
        }
        &INSTANCE
    }

    pub fn add_system(&self, system: System) -> Result<String, Error> {
        match self.db.collection("system").insert_one(system) {
            Ok(result) => Ok((result.inserted_id.to_string())),
            Err(e) => Err(Error::DbError(format!("Error adding system: {}", e))),
        }
    }

    pub fn get_systems(&self) -> Result<Vec<System>, Error> {
        let cursor = self
            .db
            .collection("system")
            .find(doc! {})
            .run()
            .map_err(|e| Error::DbError(format!("Error getting systems: {}", e)))?;
        let systems: Vec<System> = cursor
            .collect::<Result<Vec<System>, _>>()
            .map_err(|e| Error::DbError(format!("Error getting systems: {}", e)))?;
        Ok(systems)
    }
}
