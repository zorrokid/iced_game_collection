use bson::oid::ObjectId;
use lazy_static::lazy_static;
use polodb_core::{
    bson::{doc, Document},
    options::UpdateOptions,
    CollectionT, Database,
};

use crate::{
    error::Error,
    model::{
        collection_file::CollectionFile,
        model::{Emulator, Game, HasOid, Release, ReleasesByGame, Settings, System},
    },
    repository::repository::{
        CollectionFilesReadRepository, GamesReadRepository, ReleaseReadRepository,
        SystemReadRepository,
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
const COLLECTION_FILE_COLLECTION: &str = "collection_file_collection";

pub struct DatabaseWithPolo {
    db: Database,
}

impl DatabaseWithPolo {
    pub fn new(db_path: &str) -> Self {
        let db = Database::open_path(db_path).unwrap();
        Self { db }
    }

    pub fn get_instance() -> &'static Self {
        lazy_static! {
            static ref INSTANCE: DatabaseWithPolo = DatabaseWithPolo::new(COLLECTION_DATABASE_NAME);
        }
        &INSTANCE
    }

    pub fn add_system(&self, system: &System) -> Result<ObjectId, Error> {
        self.add_item(SYSTEM_COLLECTION, system)
    }

    pub fn add_game(&self, game: &Game) -> Result<ObjectId, Error> {
        self.add_item(GAME_COLLECTION, game)
    }

    pub fn add_emulator(&self, emulator: &Emulator) -> Result<ObjectId, Error> {
        self.add_item(EMULATOR_COLLECTION, emulator)
    }

    pub fn add_collection_file(&self, collection_file: &CollectionFile) -> Result<ObjectId, Error> {
        self.add_item(COLLECTION_FILE_COLLECTION, collection_file)
    }

    pub fn add_release(&self, release: &Release) -> Result<ObjectId, Error> {
        println!("Adding release: {:?}", release);
        let game_ids = &release.games;
        let release_id = self.add_item(RELEASE_COLLECTION, release)?;

        game_ids.iter().for_each(|game_id| {
            let current_values =
                self.get_with_id::<ReleasesByGame>(RELEASES_BY_GAMES_COLLECTION, game_id);

            println!("current_values: {:?}", current_values);

            match current_values {
                Ok(Some(mut releases_by_game)) => {
                    releases_by_game.release_ids.push(release_id);
                    match self.update_item(
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
                    match self.add_item(RELEASES_BY_GAMES_COLLECTION, &releases_by_game) {
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

        self.update_item(SYSTEM_COLLECTION, system, update_doc)
    }

    pub fn update_game(&self, game: &Game) -> Result<ObjectId, Error> {
        let update_doc = doc! {
            "$set": {
                "name": &game.name,
            }
        };

        self.update_item(GAME_COLLECTION, game, update_doc)
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

        self.update_item(EMULATOR_COLLECTION, emulator, update_doc)
    }

    pub fn update_release(&self, release: &Release) -> Result<ObjectId, Error> {
        println!("Updating release: {:?}", release);

        let current_release = self
            .get_release(&release.id())?
            .expect("Existing version of release not found");

        let games_in_curent_release = &current_release.games;
        let games_in_updated_release = &release.games;

        let removed_games = games_in_curent_release
            .iter()
            .filter(|game_id| !games_in_updated_release.contains(game_id))
            .collect::<Vec<&ObjectId>>();

        // TODO: is this transcation used correctly?
        let transaction = self
            .db
            .start_transaction()
            .map_err(|e| Error::DbError(e.to_string()))?;

        removed_games.iter().for_each(|game_id| {
            let current_values =
                self.get_with_id::<ReleasesByGame>(RELEASES_BY_GAMES_COLLECTION, game_id);

            match current_values {
                Ok(Some(mut releases_by_game)) => {
                    releases_by_game
                        .release_ids
                        .retain(|id| *id != release.id());
                    self.update_item(
                        RELEASES_BY_GAMES_COLLECTION,
                        &releases_by_game,
                        doc! {
                            "$set": {
                                "release_ids": releases_by_game.release_ids.clone(),
                            }
                        },
                    )
                    .expect("Error updating releases_by_game");
                }
                _ => {}
            }
        });

        let new_games = games_in_updated_release
            .iter()
            .filter(|game_id| !games_in_curent_release.contains(game_id))
            .collect::<Vec<&ObjectId>>();

        new_games.iter().for_each(|game_id| {
            let current_values =
                self.get_with_id::<ReleasesByGame>(RELEASES_BY_GAMES_COLLECTION, game_id);

            match current_values {
                Ok(Some(mut releases_by_game)) => {
                    releases_by_game.release_ids.push(release.id());
                    self.update_item(
                        RELEASES_BY_GAMES_COLLECTION,
                        &releases_by_game,
                        doc! {
                            "$set": {
                                "release_ids": releases_by_game.release_ids.clone(),
                            }
                        },
                    )
                    .expect("Error updating releases_by_game");
                }
                Ok(None) => {
                    let releases_by_game = ReleasesByGame {
                        _id: **game_id,
                        release_ids: vec![release.id()],
                    };
                    self.add_item(RELEASES_BY_GAMES_COLLECTION, &releases_by_game)
                        .expect("Error adding releases_by_game");
                }
                Err(e) => {
                    // TODO handle error
                    println!("Error: {}", e);
                }
            }
        });

        // TODO: before updating, check existing release
        // - if existing release has files, check if files are the same
        // -- delete files that are not in the new release
        // - if release has games, check if games are the same
        // -- delete game-release mapping for games that are not in the updated release

        let update_doc = doc! {
            "$set": {
                "name": &release.name,
                "system_id": &release.system_id,
                "games": &release.games,
                "files": &release.files,
            }
        };

        let result = self.update_item(RELEASE_COLLECTION, release, update_doc);

        transaction
            .commit()
            .map_err(|e| Error::DbError(e.to_string()))?;

        result
    }

    pub fn get_systems(&self) -> Result<Vec<System>, Error> {
        self.get_all_items(SYSTEM_COLLECTION)
    }

    pub fn get_all_games(&self) -> Result<Vec<Game>, Error> {
        self.get_all_items(GAME_COLLECTION)
    }

    pub fn get_emulators(&self) -> Result<Vec<Emulator>, Error> {
        self.get_all_items(EMULATOR_COLLECTION)
    }

    pub fn get_game(&self, id: &ObjectId) -> Result<Option<Game>, Error> {
        self.get_with_id(GAME_COLLECTION, id)
    }

    pub fn get_emulator(&self, id: &ObjectId) -> Result<Option<Emulator>, Error> {
        self.get_with_id(EMULATOR_COLLECTION, id)
    }

    pub fn get_system(&self, id: &ObjectId) -> Result<Option<System>, Error> {
        self.get_with_id(SYSTEM_COLLECTION, id)
    }

    pub fn get_settings(&self) -> Result<Settings, Error> {
        let settings = self.get_with_filter(SETTINGS_COLLECTION, doc! {"id": SETTINGS_ID})?;

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

    fn add_item<T>(&self, collection_name: &str, item: &T) -> Result<ObjectId, Error>
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

    fn get_all_items<T>(&self, collection_name: &str) -> Result<Vec<T>, Error>
    where
        T: for<'a> serde::Deserialize<'a>
            + serde::Serialize
            + std::marker::Sync
            + std::marker::Send
            + std::marker::Unpin,
    {
        self.get_items_with_filter(collection_name, doc! {})
    }

    fn get_items_with_filter<T>(
        &self,
        collection_name: &str,
        filter: Document,
    ) -> Result<Vec<T>, Error>
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
            .find(filter)
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

    fn get_with_id<T>(&self, collection_name: &str, id: &ObjectId) -> Result<Option<T>, Error>
    where
        T: for<'a> serde::Deserialize<'a>
            + serde::Serialize
            + std::marker::Sync
            + std::marker::Send,
    {
        self.get_with_filter(collection_name, doc! {"_id": id})
    }

    fn get_with_filter<T>(
        &self,
        collection_name: &str,
        filter: Document,
    ) -> Result<Option<T>, Error>
    where
        T: for<'a> serde::Deserialize<'a>
            + serde::Serialize
            + std::marker::Sync
            + std::marker::Send,
    {
        let res = self
            .db
            .collection::<T>(collection_name)
            .find_one(filter)
            .map_err(|e| Error::DbError(format!("Error getting item: {}", e)))?;
        Ok(res)
    }

    pub fn delete_emulator(&self, id: &ObjectId) -> Result<(), Error> {
        self.delete_item::<Emulator>(EMULATOR_COLLECTION, id)
    }

    pub fn delete_game(&self, id: &ObjectId) -> Result<(), Error> {
        if self.is_game_in_release(id)? {
            Err(Error::DbError(
                "Game cannot be deleted because it is used in a release".to_string(),
            ))
        } else {
            self.delete_item::<Game>(GAME_COLLECTION, id)
        }
    }

    pub fn delete_system(&self, id: &ObjectId) -> Result<(), Error> {
        if self.is_system_in_release(id)? {
            Err(Error::DbError(
                "System cannot be deleted because it is used in a release".to_string(),
            ))
        } else {
            self.delete_item::<System>(SYSTEM_COLLECTION, id)
        }
    }

    pub fn delete_release(&self, id: &ObjectId) -> Result<(), Error> {
        let release = self.get_release(id)?.expect("Release not found");
        if release.files.is_empty() {
            self.delete_release_from_games(&release)?;
            self.delete_item::<Release>(RELEASE_COLLECTION, id)
        } else {
            Err(Error::DbError(
                "Release cannot be deleted because it has files".to_string(),
            ))
        }
    }

    pub fn delete_release_from_games(&self, release: &Release) -> Result<(), Error> {
        let game_ids = &release.games;
        let result = game_ids.iter().try_for_each(|game_id| {
            let current_values =
                self.get_with_id::<ReleasesByGame>(RELEASES_BY_GAMES_COLLECTION, game_id)?;

            match current_values {
                Some(mut releases_by_game) => {
                    releases_by_game
                        .release_ids
                        .retain(|id| *id != release.id());
                    self.update_item(
                        RELEASES_BY_GAMES_COLLECTION,
                        &releases_by_game,
                        doc! {
                            "$set": {
                                "release_ids": releases_by_game.release_ids.clone(),
                            }
                        },
                    )?;
                }
                _ => {}
            }
            Ok(())
        });
        result
    }

    fn delete_item<T>(&self, collection_name: &str, id: &ObjectId) -> Result<(), Error>
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

impl ReleaseReadRepository for DatabaseWithPolo {
    fn get_release(&self, id: &ObjectId) -> Result<Option<Release>, Error> {
        self.get_with_id(RELEASE_COLLECTION, id)
    }
    fn get_releases_with_game(&self, id: &ObjectId) -> Result<Vec<Release>, Error> {
        let releases_by_game =
            self.get_with_id::<ReleasesByGame>(RELEASES_BY_GAMES_COLLECTION, id)?;

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
}

impl GamesReadRepository for DatabaseWithPolo {
    fn get_games(&self, ids: &Vec<ObjectId>) -> Result<Vec<Game>, Error> {
        self.get_items_with_filter(GAME_COLLECTION, doc! {"_id": {"$in": ids}})
    }
    fn get_all_games(&self) -> Result<Vec<Game>, Error> {
        self.get_all_items(GAME_COLLECTION)
    }
    fn is_game_in_release(&self, game_id: &ObjectId) -> Result<bool, Error> {
        let releases_by_game =
            self.get_with_id::<ReleasesByGame>(RELEASES_BY_GAMES_COLLECTION, game_id)?;

        match releases_by_game {
            Some(releases_by_game) => Ok(!releases_by_game.release_ids.is_empty()),
            None => Ok(false),
        }
    }
}

impl CollectionFilesReadRepository for DatabaseWithPolo {
    fn get_collection_files(&self, ids: &Vec<ObjectId>) -> Result<Vec<CollectionFile>, Error> {
        self.get_items_with_filter(COLLECTION_FILE_COLLECTION, doc! {"_id": {"$in": ids}})
    }
}

impl SystemReadRepository for DatabaseWithPolo {
    fn get_system(&self, id: &ObjectId) -> Result<Option<System>, Error> {
        self.get_with_id(SYSTEM_COLLECTION, id)
    }
    fn is_system_in_release(&self, system_id: &ObjectId) -> Result<bool, Error> {
        let filter = doc! {"system_id": system_id};
        let release = self
            .db
            .collection::<Release>(RELEASE_COLLECTION)
            .find_one(filter)
            .map_err(|e| Error::DbError(format!("Error finding a release: {}", e)))?;
        Ok(release.is_some())
    }
    fn get_all_systems(&self) -> Result<Vec<System>, Error> {
        self.get_all_items(SYSTEM_COLLECTION)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::model::System;

    #[test]
    fn test_add_system() {
        let test_db = super::DatabaseWithPolo::new("test.db");
        let system = System {
            _id: None,
            name: "Test system".to_string(),
        };
        let id = test_db.add_system(&system).unwrap();

        let system_from_db = test_db.get_system(&id).unwrap().unwrap();
        assert_eq!(system_from_db.name, system.name);
        std::fs::remove_dir_all("test.db").unwrap();
    }
}
