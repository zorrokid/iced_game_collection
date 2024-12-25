use bson::oid::ObjectId;
use lazy_static::lazy_static;
use polodb_core::{
    bson::{doc, Document},
    options::UpdateOptions,
    CollectionT, Database,
};

use crate::{
    error::Error,
    model::model::{
        Emulator, Game, GameListModel, HasId, HasOid, Release, ReleasesByGame, Settings, System,
    },
};

const COLLECTION_DATABASE_NAME: &str = "iced_game_collection.db";
const SYSTEM_COLLECTION: &str = "system";
const GAME_COLLECTION: &str = "game";
const EMULATOR_COLLECTION: &str = "emulator";
const SETTINGS_COLLECTION: &str = "settings";
const RELEASE_COLLECTION: &str = "release";
const SETTINGS_ID: &str = "settings";
const RELEASES_BY_GAMES_COLLECTION: &str = "releases_by_games";

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

    pub fn add_game(&self, game: &Game) -> Result<ObjectId, Error> {
        self.add_item_new(GAME_COLLECTION, game)
    }

    pub fn add_emulator(&self, emulator: &Emulator) -> Result<ObjectId, Error> {
        self.add_item_new(EMULATOR_COLLECTION, emulator)
    }

    pub fn add_release(&self, release: &Release) -> Result<ObjectId, Error> {
        println!("Adding release: {:?}", release);
        let game_ids = &release.games;
        let release_id = self.add_item_new(RELEASE_COLLECTION, release)?;

        game_ids.iter().for_each(|game_id| {
            let current_values =
                self.get_item_new::<ReleasesByGame>(RELEASES_BY_GAMES_COLLECTION, game_id);

            println!("current_values: {:?}", current_values);

            match current_values {
                Ok(Some(mut releases_by_game)) => {
                    releases_by_game.release_ids.push(release_id);
                    match self.update_item_new(
                        RELEASES_BY_GAMES_COLLECTION,
                        &releases_by_game,
                        doc! {
                            "$set": {
                                "release_ids": releases_by_game.release_ids.clone(),
                            }
                        },
                    ) {
                        Ok(_) => {}
                        Err(e) => {
                            // TODO handle error
                            println!("Error: {}", e);
                        }
                    }
                }
                Ok(None) => {
                    let releases_by_game = ReleasesByGame {
                        _id: game_id.clone(),
                        release_ids: vec![release_id],
                    };
                    match self.add_item_new(RELEASES_BY_GAMES_COLLECTION, &releases_by_game) {
                        Ok(id) => {
                            println!("Added releases_by_game: {:?}", id);
                        }
                        Err(e) => {
                            // TODO handle error
                            println!("Error: {}", e);
                        }
                    }
                }
                Err(e) => {
                    // TODO handle error
                    println!("Error: {}", e);
                }
            }
        });
        Ok(release_id)
    }

    pub fn add_or_update_settings(&self, settings: &Settings) -> Result<String, Error> {
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

    pub fn update_system(&self, system: &System) -> Result<ObjectId, Error> {
        let update_doc = doc! {
            "$set": {
                "name": &system.name,
            }
        };

        self.update_item_new(SYSTEM_COLLECTION, system, update_doc)
    }

    pub fn update_game(&self, game: &Game) -> Result<ObjectId, Error> {
        let update_doc = doc! {
            "$set": {
                "name": &game.name,
            }
        };

        self.update_item_new(GAME_COLLECTION, game, update_doc)
    }

    pub fn update_emulator(&self, emulator: &Emulator) -> Result<ObjectId, Error> {
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

        self.update_item_new(EMULATOR_COLLECTION, emulator, update_doc)
    }

    pub fn update_release(&self, release: &Release) -> Result<ObjectId, Error> {
        println!("Updating release: {:?}", release);
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

        self.update_item_new(RELEASE_COLLECTION, release, update_doc)
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

    pub fn get_game(&self, id: &ObjectId) -> Result<Option<Game>, Error> {
        self.get_item_new(GAME_COLLECTION, id)
    }

    pub fn get_emulator(&self, id: &ObjectId) -> Result<Option<Emulator>, Error> {
        self.get_item_new(EMULATOR_COLLECTION, id)
    }

    pub fn get_release(&self, id: &ObjectId) -> Result<Option<Release>, Error> {
        self.get_item_new(RELEASE_COLLECTION, id)
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
            Ok(result) => {
                // ObjectId("676337e2233281af03ebe19f")
                println!("Got Inserted id: {:?}", result.inserted_id.as_str());
                Ok(result.inserted_id.to_string())
            }
            Err(e) => Err(Error::DbError(format!("Error adding item: {}", e))),
        }
    }

    fn add_item_new<T>(&self, collection_name: &str, item: &T) -> Result<ObjectId, Error>
    where
        T: serde::Serialize,
    {
        match self.db.collection::<T>(collection_name).insert_one(item) {
            Ok(result) => {
                if let Some(oid) = result.inserted_id.as_object_id() {
                    Ok(oid)
                } else {
                    Err(Error::DbError("Error getting inserted id".to_string()))
                }
            }
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

    fn update_item_new<T>(
        &self,
        collection_name: &str,
        item: &T,
        update_document: Document,
    ) -> Result<ObjectId, Error>
    where
        T: serde::Serialize,
        T: HasOid,
    {
        match self
            .db
            .collection::<T>(collection_name)
            .update_one(doc! {"_id": item.id()}, update_document)
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

    pub fn get_releases_with_game(&self, id: &ObjectId) -> Result<Vec<Release>, Error> {
        let releases_by_game =
            self.get_item_new::<ReleasesByGame>(RELEASES_BY_GAMES_COLLECTION, id)?;

        println!("releases_by_game: {:?}", releases_by_game);

        let relese_ids = match releases_by_game {
            Some(releases_by_game) => releases_by_game.release_ids,
            None => vec![],
        };

        if let Ok(cursor) = self
            .db
            .collection(RELEASE_COLLECTION)
            .find(doc! {"_id": {"$in": relese_ids}})
            .run()
            .map_err(|e| Error::DbError(format!("Error getting releases with game: {}", e)))
        {
            let releases_with_game: Vec<Release> = cursor
                .collect::<Result<Vec<Release>, _>>()
                .map_err(|e| Error::DbError(format!("Error getting releases with game: {}", e)))
                .unwrap_or(vec![]);
            println!("collected {:?}", releases_with_game);
            Ok(releases_with_game)
        } else {
            println!("Didn't find any releases");
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

    fn get_item_new<T>(&self, collection_name: &str, id: &ObjectId) -> Result<Option<T>, Error>
    where
        T: for<'a> serde::Deserialize<'a>
            + serde::Serialize
            + std::marker::Sync
            + std::marker::Send,
    {
        let res = self
            .db
            .collection::<T>(collection_name)
            .find_one(doc! {"_id": id})
            .map_err(|e| Error::DbError(format!("Error getting item: {}", e)))?;
        Ok(res)
    }

    pub fn to_game_list_model(&self) -> Result<Vec<GameListModel>, Error> {
        let games = self.get_games()?;
        let mut list_models: Vec<GameListModel> = games.iter().map(GameListModel::from).collect();

        for game in &mut list_models {
            let releases_with_game = self.get_releases_with_game(&game.id())?;
            game.can_delete = releases_with_game.is_empty();
        }
        Ok(list_models)
    }

    pub fn delete_emulator(&self, id: &ObjectId) -> Result<(), Error> {
        self.delete_item_new::<Emulator>(EMULATOR_COLLECTION, id)
    }

    pub fn delete_game(&self, id: &ObjectId) -> Result<(), Error> {
        // TODO: game cannot be deleted if used in a release
        self.delete_item_new::<Game>(GAME_COLLECTION, id)
    }

    pub fn delete_system(&self, id: &ObjectId) -> Result<(), Error> {
        // TODO: system cannot be deleted if used in a realase or emulator
        self.delete_item_new::<System>(SYSTEM_COLLECTION, id)
    }

    fn delete_item<T>(&self, collection_name: &str, id: &str) -> Result<(), Error>
    where
        T: HasId + serde::Serialize,
    {
        match self
            .db
            .collection::<T>(collection_name)
            .delete_one(doc! {"id": id})
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::DbError(format!("Error deleting item: {}", e))),
        }
    }

    fn delete_item_new<T>(&self, collection_name: &str, id: &ObjectId) -> Result<(), Error>
    where
        T: serde::Serialize,
    {
        match self
            .db
            .collection::<T>(collection_name)
            .delete_one(doc! {"_id": id})
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::DbError(format!("Error deleting item: {}", e))),
        }
    }
}
