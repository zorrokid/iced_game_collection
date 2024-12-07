use lazy_static::lazy_static;
use polodb_core::{
    bson::{doc, document, Document},
    CollectionT, Database,
};

use crate::{
    error::Error,
    model::model::{Game, HasId, System},
};

const COLLECTION_DATABASE_NAME: &str = "iced_game_collection.db";
const SYSTEM_COLLECTION: &str = "system";
const GAME_COLLECTION: &str = "game";

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
        self.add_item(SYSTEM_COLLECTION, system)
    }

    pub fn add_game(&self, game: Game) -> Result<String, Error> {
        self.add_item(GAME_COLLECTION, game)
    }

    pub fn update_system(&self, system: System) -> Result<(), Error> {
        let update_doc = doc! {
            "$set": {
                "name": system.name.clone(),
            }
        };

        self.update_item(SYSTEM_COLLECTION, system, update_doc)
    }

    pub fn update_game(&self, game: Game) -> Result<(), Error> {
        let update_doc = doc! {
            "$set": {
                "name": game.name.clone(),
            }
        };

        self.update_item(GAME_COLLECTION, game, update_doc)
    }

    pub fn get_systems(&self) -> Result<Vec<System>, Error> {
        self.get_items(SYSTEM_COLLECTION)
    }

    pub fn get_games(&self) -> Result<Vec<Game>, Error> {
        self.get_items(GAME_COLLECTION)
    }

    fn add_item<T>(&self, collection_name: &str, item: T) -> Result<String, Error>
    where
        T: serde::Serialize,
    {
        match self.db.collection::<T>(collection_name).insert_one(item) {
            Ok(result) => Ok(result.inserted_id.to_string()),
            Err(e) => Err(Error::DbError(format!("Error adding item: {}", e))),
        }
    }

    fn update_item<T>(
        &self,
        collection_name: &str,
        item: T,
        update_document: Document,
    ) -> Result<(), Error>
    where
        T: serde::Serialize,
        T: HasId,
    {
        match self
            .db
            .collection::<T>(collection_name)
            .update_one(doc! {"id": item.id()}, update_document)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::DbError(format!("Error updating system: {}", e))),
        }
    }

    fn get_items<T>(&self, collection_name: &str) -> Result<Vec<T>, Error>
    where
        T: for<'a> serde::Deserialize<'a>
            + serde::Serialize
            + std::marker::Sync
            + std::marker::Send
            + std::marker::Unpin,
    {
        if let Ok(cursor) = self
            .db
            .collection(collection_name)
            .find(doc! {})
            .run()
            .map_err(|e| Error::DbError(format!("Error getting items: {}", e)))
        {
            let items: Vec<T> = cursor
                .collect::<Result<Vec<T>, _>>()
                .map_err(|e| Error::DbError(format!("Error getting items: {}", e)))?;
            Ok(items)
        } else {
            Ok(vec![])
        }
    }
}
