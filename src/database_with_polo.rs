use lazy_static::lazy_static;
use polodb_core::{
    bson::{doc, Document},
    options::UpdateOptions,
    CollectionT, Database,
};

use crate::{
    error::Error,
    model::model::{Emulator, Game, HasId, Release, Settings, System},
};

const COLLECTION_DATABASE_NAME: &str = "iced_game_collection.db";
const SYSTEM_COLLECTION: &str = "system";
const GAME_COLLECTION: &str = "game";
const EMULATOR_COLLECTION: &str = "emulator";
const SETTINGS_COLLECTION: &str = "settings";
const RELEASE_COLLECTION: &str = "release";
const SETTINGS_ID: &str = "settings";

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

    pub fn add_system(&self, system: &System) -> Result<String, Error> {
        self.add_item(SYSTEM_COLLECTION, system)
    }

    pub fn add_game(&self, game: &Game) -> Result<String, Error> {
        self.add_item(GAME_COLLECTION, game)
    }

    pub fn add_emulator(&self, emulator: &Emulator) -> Result<String, Error> {
        self.add_item(EMULATOR_COLLECTION, emulator)
    }

    pub fn add_release(&self, release: &Release) -> Result<String, Error> {
        self.add_item(RELEASE_COLLECTION, release)
    }

    pub fn add_or_update_settings(&self, settings: &Settings) -> Result<String, Error> {
        //self.add_item(SETTINGS_COLLECTION, settings)
        let filter = doc! {"id": SETTINGS_ID};
        let update_doc = doc! {
            "$set": {
                "collection_root_dir": &settings.collection_root_dir,
            }
        };
        match self
            .db
            .collection::<Settings>(SETTINGS_COLLECTION)
            .update_one_with_options(
                filter,
                update_doc,
                UpdateOptions::builder().upsert(true).build(),
            ) {
            Ok(_) => Ok(SETTINGS_ID.to_string()),
            Err(e) => Err(Error::DbError(format!("Error updating settings: {}", e))),
        }
    }

    pub fn update_system(&self, system: &System) -> Result<String, Error> {
        let update_doc = doc! {
            "$set": {
                "name": &system.name,
            }
        };

        self.update_item(SYSTEM_COLLECTION, system, update_doc)
    }

    pub fn update_game(&self, game: &Game) -> Result<String, Error> {
        let update_doc = doc! {
            "$set": {
                "name": &game.name,
            }
        };

        self.update_item(GAME_COLLECTION, game, update_doc)
    }

    pub fn update_emulator(&self, emulator: &Emulator) -> Result<String, Error> {
        let update_doc = doc! {
            "$set": {
                "name": &emulator.name,
                "executable": &emulator.executable,
                "arguments": &emulator.arguments,
                "system_id": &emulator.system_id,
                "extract_files": emulator.extract_files,
                "supported_file_type_extensions": emulator.supported_file_type_extensions.clone(),
            }
        };

        self.update_item(EMULATOR_COLLECTION, emulator, update_doc)
    }

    pub fn update_release(&self, release: &Release) -> Result<String, Error> {
        let update_doc = doc! {
                    "$set": {
                        "name": &release.name,
                        "system_id": &release.system_id,
                        "games": &release.games,
        // TODO: should collection files be stored in another collection?
        // and how about files in collection file?
                        "files": &release.files,
                    }
                };

        self.update_item(RELEASE_COLLECTION, release, update_doc)
    }

    pub fn get_systems(&self) -> Result<Vec<System>, Error> {
        self.get_items(SYSTEM_COLLECTION)
    }

    pub fn get_games(&self) -> Result<Vec<Game>, Error> {
        self.get_items(GAME_COLLECTION)
    }

    pub fn get_emulators(&self) -> Result<Vec<Emulator>, Error> {
        self.get_items(EMULATOR_COLLECTION)
    }

    pub fn get_game(&self, id: &str) -> Result<Option<Game>, Error> {
        self.get_item(GAME_COLLECTION, id)
    }

    pub fn get_emulator(&self, id: &str) -> Result<Option<Emulator>, Error> {
        self.get_item(EMULATOR_COLLECTION, id)
    }

    pub fn get_release(&self, id: &str) -> Result<Option<Release>, Error> {
        self.get_item(RELEASE_COLLECTION, id)
    }

    pub fn get_settings(&self) -> Result<Settings, Error> {
        let settings = self.get_item(SETTINGS_COLLECTION, SETTINGS_ID)?;

        // if settings does not exist, create default settings
        match settings {
            Some(settings) => Ok(settings),
            None => {
                let default_settings = Settings {
                    id: SETTINGS_ID.to_string(),
                    collection_root_dir: "".to_string(),
                };
                self.add_or_update_settings(&default_settings)?;
                Ok(default_settings)
            }
        }
    }

    fn add_item<T>(&self, collection_name: &str, item: &T) -> Result<String, Error>
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
        item: &T,
        update_document: Document,
    ) -> Result<String, Error>
    where
        T: serde::Serialize,
        T: HasId,
    {
        match self
            .db
            .collection::<T>(collection_name)
            .update_one(doc! {"id": item.id()}, update_document)
        {
            Ok(_) => Ok(item.id()),
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

    fn get_item<T>(&self, collection_name: &str, id: &str) -> Result<Option<T>, Error>
    where
        T: for<'a> serde::Deserialize<'a>
            + serde::Serialize
            + std::marker::Sync
            + HasId
            + std::marker::Send,
    {
        let res = self
            .db
            .collection::<T>(collection_name)
            .find_one(doc! {"id": id})
            .map_err(|e| Error::DbError(format!("Error getting item: {}", e)))?;
        Ok(res)
    }
}
