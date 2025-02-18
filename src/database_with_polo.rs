use bson::oid::ObjectId;
use lazy_static::lazy_static;
use polodb_core::{
    bson::{doc, Document},
    options::UpdateOptions,
    CollectionT, Database, Transaction,
};

use crate::{
    error::Error,
    model::{
        collection_file::CollectionFile,
        model::{
            Emulator, Franchise, Game, HasOid, Release, ReleasesByFile, ReleasesByGame, Settings,
            System,
        },
    },
    repository::repository::{
        CollectionFilesReadRepository, FranchisReadRepository, GamesReadRepository,
        ReleaseReadRepository, SystemReadRepository,
    },
};

const COLLECTION_DATABASE_NAME: &str = "iced_game_collection.db";
const SYSTEM_COLLECTION: &str = "system";
const GAME_COLLECTION: &str = "game";
const FRANCHISE_COLLECTION: &str = "franchise";
const EMULATOR_COLLECTION: &str = "emulator";
const SETTINGS_COLLECTION: &str = "settings";
const RELEASE_COLLECTION: &str = "release";
const SETTINGS_ID: &str = "settings";
const RELEASES_BY_GAMES_COLLECTION: &str = "releases_by_games";
const RELEASES_BY_FILES_COLLECTION: &str = "releases_by_files";
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

    pub fn delete_collection_file(&self, id: &ObjectId) -> Result<(), Error> {
        if self.is_collection_file_in_release(id)? {
            Err(Error::DbError(
                "Collection file cannot be deleted because it is used in a release".to_string(),
            ))
        } else {
            self.delete_item::<CollectionFile>(COLLECTION_FILE_COLLECTION, id)
        }
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
                "notes": &system.notes,
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
                "notes": &emulator.notes
            }
        };

        self.update_item(EMULATOR_COLLECTION, emulator, update_doc)
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

    fn add_item_in_transaction<T>(
        &self,
        collection_name: &str,
        item: &T,
        transaction: &Transaction,
    ) -> Result<ObjectId, Error>
    where
        T: serde::Serialize,
    {
        match transaction
            .collection::<T>(collection_name)
            .insert_one(item)
        {
            Ok(result) => {
                if let Some(oid) = result.inserted_id.as_object_id() {
                    println!("inserted id: {:?}", oid);
                    Ok(oid)
                } else {
                    Err(Error::DbError("Error getting inserted id".to_string()))
                }
            }
            Err(e) => Err(Error::DbError(format!("Error adding item: {}", e))),
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

        // files need to be deleted before release can be deleted
        if release.files.is_empty() {
            let transaction = self
                .db
                .start_transaction()
                .map_err(|e| Error::DbError(e.to_string()))?;

            if let Err(err) = self.delete_release_from_games(&release, &transaction) {
                transaction
                    .rollback()
                    .map_err(|e| Error::DbError(e.to_string()))?;
                return Err(err);
            }

            if let Err(err) = self.delete_item::<Release>(RELEASE_COLLECTION, id) {
                transaction
                    .rollback()
                    .map_err(|e| Error::DbError(e.to_string()))?;
                return Err(err);
            }

            transaction
                .commit()
                .map_err(|e| Error::DbError(e.to_string()))?;

            Ok(())
        } else {
            Err(Error::DbError(
                "Release cannot be deleted because it has files".to_string(),
            ))
        }
    }

    fn delete_release_from_games(
        &self,
        release: &Release,
        transaction: &Transaction,
    ) -> Result<(), Error> {
        let game_ids = &release.games;

        for game_id in game_ids {
            let current_values = transaction
                .collection::<ReleasesByGame>(RELEASES_BY_GAMES_COLLECTION)
                .find_one(doc! {"_id": game_id})
                .map_err(|e| Error::DbError(format!("Error getting item: {}", e)))?;

            if let Some(mut releases_by_game) = current_values {
                releases_by_game
                    .release_ids
                    .retain(|id| *id != release.id());

                if releases_by_game.release_ids.is_empty() {
                    transaction
                        .collection::<ReleasesByGame>(RELEASES_BY_GAMES_COLLECTION)
                        .delete_one(doc! {"_id": game_id})
                        .map_err(|e| Error::DbError(format!("Error deleting item: {}", e)))?;
                } else {
                    transaction
                        .collection::<ReleasesByGame>(RELEASES_BY_GAMES_COLLECTION)
                        .update_one(
                            doc! {"_id": releases_by_game._id},
                            doc! {
                                "$set": {
                                    "release_ids": releases_by_game.release_ids.clone(),
                                }
                            },
                        )
                        .map_err(|e| Error::DbError(format!("Error updating item: {}", e)))?;
                }
            }
        }

        Ok(())
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

    /// Adds a release to the database. Also updates the references to the release in the games and
    /// files collections.
    pub fn add_release(&self, release: &Release) -> Result<ObjectId, Error> {
        let game_ids = &release.games;
        let file_ids = &release.files;

        println!("game_ids: {:?}", game_ids);
        println!("file_ids: {:?}", file_ids);

        println!("Starting transaction");

        let transaction = self
            .db
            .start_transaction()
            .map_err(|e| Error::DbError(e.to_string()))?;

        let release_insert_result =
            self.add_item_in_transaction(RELEASE_COLLECTION, release, &transaction);

        match release_insert_result {
            Ok(release_id) => {
                println!("release_id: {:?}", release_id);
                if let Err(e) =
                    self.update_release_references(&release_id, file_ids, game_ids, &transaction)
                {
                    transaction
                        .rollback()
                        .map_err(|e| Error::DbError(e.to_string()))?;
                    return Err(e);
                }
                println!("Starting commit");
                transaction
                    .commit()
                    .map_err(|e| Error::DbError(e.to_string()))?;
                Ok(release_id)
            }
            Err(e) => {
                transaction
                    .rollback()
                    .map_err(|e| Error::DbError(e.to_string()))?;
                Err(Error::DbError(e.to_string()))
            }
        }
    }

    pub fn update_release(&self, release: &Release) -> Result<ObjectId, Error> {
        let current_release = self
            .get_release(&release.id())?
            .expect("Existing version of release not found");

        let transaction = self
            .db
            .start_transaction()
            .map_err(|e| Error::DbError(e.to_string()))?;

        let games_in_curent_release = &current_release.games;
        let games_in_updated_release = &release.games;

        let removed_games = games_in_curent_release
            .iter()
            .filter(|game_id| !games_in_updated_release.contains(game_id))
            .collect::<Vec<&ObjectId>>();

        let new_games = games_in_updated_release
            .iter()
            .filter(|game_id| !games_in_curent_release.contains(game_id))
            .collect::<Vec<&ObjectId>>();

        let files_in_current_release = &current_release.files;
        let files_in_updated_release = &release.files;

        let removed_files = files_in_current_release
            .iter()
            .filter(|file_id| !files_in_updated_release.contains(file_id))
            .collect::<Vec<&ObjectId>>();

        let new_files = files_in_updated_release
            .iter()
            .filter(|file_id| !files_in_current_release.contains(file_id))
            .collect::<Vec<&ObjectId>>();

        if let Err(e) = self.update_release_references_2(
            &release.id(),
            &new_files,
            &new_games,
            &removed_files,
            &removed_games,
            &transaction,
        ) {
            transaction
                .rollback()
                .map_err(|e| Error::DbError(e.to_string()))?;
            return Err(e);
        }

        let update_doc = doc! {
            "$set": {
                "name": &release.name,
                "system_id": &release.system_id,
                "games": &release.games,
                "files": &release.files,
            }
        };

        match transaction
            .collection::<Release>(RELEASE_COLLECTION)
            .update_one(doc! {"_id": release.id()}, update_doc)
        {
            Ok(_) => {
                transaction
                    .commit()
                    .map_err(|e| Error::DbError(e.to_string()))?;
                Ok(release.id())
            }
            Err(e) => {
                transaction
                    .rollback()
                    .map_err(|e| Error::DbError(e.to_string()))?;
                Err(Error::DbError(e.to_string()))
            }
        }
    }

    // TODO: merge the two following functions
    fn update_release_references_2(
        &self,
        release_id: &ObjectId,
        new_file_ids: &Vec<&ObjectId>,
        new_game_ids: &Vec<&ObjectId>,
        removed_file_ids: &Vec<&ObjectId>,
        removed_game_ids: &Vec<&ObjectId>,
        transaction: &Transaction,
    ) -> Result<(), Error> {
        for game_id in removed_game_ids {
            self.update_removed_games_from_release(game_id, release_id, transaction)?;
        }
        for file_id in removed_file_ids {
            self.update_removed_files_from_release(file_id, release_id, transaction)?;
        }
        for game_id in new_game_ids {
            self.add_or_update_releases_by_game(game_id, release_id, transaction)?;
        }
        for file_id in new_file_ids {
            self.add_or_update_releases_by_file(file_id, release_id, transaction)?;
        }
        Ok(())
    }

    fn update_release_references(
        &self,
        release_id: &ObjectId,
        file_ids: &Vec<ObjectId>,
        game_ids: &Vec<ObjectId>,
        transaction: &Transaction,
    ) -> Result<(), Error> {
        for game_id in game_ids {
            self.add_or_update_releases_by_game(game_id, release_id, transaction)?;
        }
        for file_id in file_ids {
            self.add_or_update_releases_by_file(file_id, release_id, transaction)?;
        }
        Ok(())
    }

    fn add_or_update_releases_by_game(
        &self,
        game_id: &ObjectId,
        release_id: &ObjectId,
        transaction: &Transaction,
    ) -> Result<(), Error> {
        println!("add_or_update_releases_by_game");

        let current_releases_by_game = transaction
            .collection::<ReleasesByGame>(RELEASES_BY_GAMES_COLLECTION)
            .find_one(doc! {"_id":  game_id})
            .map_err(|e| Error::DbError(format!("Error getting releases by game {}", e)))?;

        match current_releases_by_game {
            Some(mut releases_by_game) => {
                releases_by_game.release_ids.push(*release_id);
                println!("releases_by_game: {:?}", releases_by_game);
                transaction
                    .collection::<ReleasesByGame>(RELEASES_BY_GAMES_COLLECTION)
                    .update_one(
                        doc! {"_id": game_id},
                        doc! {"$set": {"release_ids": releases_by_game.release_ids.clone()}},
                    )
                    .map_err(|err| {
                        Error::DbError(format!("Error updating releases by game {}", err))
                    })?;
            }
            None => {
                let releases_by_game = ReleasesByGame {
                    _id: *game_id,
                    release_ids: vec![*release_id],
                };
                println!("releases_by_game: {:?}", releases_by_game);
                transaction
                    .collection::<ReleasesByGame>(RELEASES_BY_GAMES_COLLECTION)
                    .insert_one(&releases_by_game)
                    .map_err(|err| {
                        Error::DbError(format!("Error updating releases by game {}", err))
                    })?;
            }
        };
        Ok(())
    }

    fn add_or_update_releases_by_file(
        &self,
        file_id: &ObjectId,
        release_id: &ObjectId,
        transaction: &Transaction,
    ) -> Result<(), Error> {
        let current_releases_by_file = transaction
            .collection::<ReleasesByFile>(RELEASES_BY_FILES_COLLECTION)
            .find_one(doc! {"_id":  file_id})
            .map_err(|e| Error::DbError(format!("Error getting releases by file {}", e)))?;

        match current_releases_by_file {
            Some(mut releases_by_file) => {
                releases_by_file.release_ids.push(*release_id);
                transaction
                    .collection::<ReleasesByFile>(RELEASES_BY_FILES_COLLECTION)
                    .update_one(
                        doc! {"_id": file_id},
                        doc! {"$set": {"release_ids": releases_by_file.release_ids.clone()}},
                    )
                    .map_err(|err| {
                        Error::DbError(format!("Error updating releases by file {}", err))
                    })?;
            }
            None => {
                let releases_by_file = ReleasesByFile {
                    _id: *file_id,
                    release_ids: vec![*release_id],
                };
                transaction
                    .collection::<ReleasesByFile>(RELEASES_BY_FILES_COLLECTION)
                    .insert_one(&releases_by_file)
                    .map_err(|err| {
                        Error::DbError(format!("Error updating releases by file {}", err))
                    })?;
            }
        };
        Ok(())
    }

    fn update_removed_games_from_release(
        &self,
        game_id: &ObjectId,
        release_id: &ObjectId,
        transaction: &Transaction,
    ) -> Result<(), Error> {
        let games = transaction
            .collection::<ReleasesByGame>(RELEASES_BY_GAMES_COLLECTION)
            .find_one(doc! {"_id": game_id})
            .map_err(|e| Error::DbError(e.to_string()))?;

        match games {
            Some(mut releases_by_game) => {
                releases_by_game.release_ids.retain(|id| *id != *release_id);

                transaction
                    .collection::<ReleasesByGame>(RELEASES_BY_GAMES_COLLECTION)
                    .update_one(
                        doc! {"_id": game_id},
                        doc! {
                            "$set": {
                                "release_ids": releases_by_game.release_ids.clone(),
                            }
                        },
                    )
                    .map_err(|e| Error::DbError(e.to_string()))?;
            }
            None => {
                return Err(Error::DbError("Game not found".to_string()));
            }
        };
        Ok(())
    }

    fn update_removed_files_from_release(
        &self,
        file_id: &ObjectId,
        release_id: &ObjectId,
        transaction: &Transaction,
    ) -> Result<(), Error> {
        let releases = transaction
            .collection::<ReleasesByFile>(RELEASES_BY_FILES_COLLECTION)
            .find_one(doc! {"_id": file_id})
            .map_err(|e| Error::DbError(e.to_string()))?;

        match releases {
            Some(mut releases_by_file) => {
                releases_by_file.release_ids.retain(|id| *id != *release_id);
                transaction
                    .collection::<ReleasesByFile>(RELEASES_BY_FILES_COLLECTION)
                    .update_one(
                        doc! {"_id": *file_id},
                        doc! {
                            "$set": {
                                "release_ids": releases_by_file.release_ids.clone(),
                            }
                        },
                    )
                    .map_err(|e| Error::DbError(e.to_string()))?;
            }
            None => {
                return Err(Error::DbError("File not found".to_string()));
            }
        };
        Ok(())
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
    fn get_releases_by_game(&self, game_id: &ObjectId) -> Result<Option<ReleasesByGame>, Error> {
        self.get_with_id(RELEASES_BY_GAMES_COLLECTION, game_id)
    }
}

impl FranchisReadRepository for DatabaseWithPolo {
    fn get_all_franchises(&self) -> Result<Vec<Franchise>, Error> {
        self.get_all_items(FRANCHISE_COLLECTION)
    }
    fn add_franchise(&self, name: &str) -> Result<ObjectId, Error> {
        let franchise = Franchise {
            _id: None,
            name: name.to_string(),
        };
        let id = self.add_item(FRANCHISE_COLLECTION, &franchise)?;
        Ok(id)
    }
}

impl CollectionFilesReadRepository for DatabaseWithPolo {
    fn get_collection_files(&self, ids: &Vec<ObjectId>) -> Result<Vec<CollectionFile>, Error> {
        self.get_items_with_filter(COLLECTION_FILE_COLLECTION, doc! {"_id": {"$in": ids}})
    }
    fn is_collection_file_in_release(&self, collection_file_id: &ObjectId) -> Result<bool, Error> {
        let releases_by_file =
            self.get_with_id::<ReleasesByFile>(RELEASES_BY_FILES_COLLECTION, collection_file_id)?;

        match releases_by_file {
            Some(releases_by_file) => Ok(!releases_by_file.release_ids.is_empty()),
            None => Ok(false),
        }
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
    use bson::oid::ObjectId;

    use crate::{
        database_with_polo::DatabaseWithPolo,
        model::{
            collection_file::{CollectionFile, CollectionFileType, FileInfo},
            model::{Game, Release, System},
        },
        repository::repository::{GamesReadRepository, ReleaseReadRepository},
    };

    fn create_test_system() -> System {
        System {
            _id: None,
            name: "Test system".to_string(),
            notes: None,
        }
    }

    fn create_test_game() -> Game {
        Game {
            _id: None,
            name: "Test game".to_string(),
            franchise_id: None,
        }
    }

    fn create_test_collection_file() -> CollectionFile {
        CollectionFile {
            _id: None,
            original_file_name: "Test file.zip".to_string(),
            is_zip: true,
            files: Some(vec![FileInfo {
                name: "Test file.disk".to_string(),
                checksum: "checksum".to_string(),
            }]),
            collection_file_type: CollectionFileType::DiskImage,
        }
    }

    fn create_test_release(
        system_id: ObjectId,
        games: Vec<ObjectId>,
        files: Vec<ObjectId>,
    ) -> Release {
        Release {
            _id: None,
            name: "Test release".to_string(),
            system_id: Some(system_id),
            games,
            files,
        }
    }

    #[test]
    fn test_add_system() {
        let test_db_name = "test_add_system.db";
        let test_db = DatabaseWithPolo::new(test_db_name);
        let system = create_test_system();
        let id = test_db.add_system(&system).unwrap();

        let system_from_db = test_db.get_system(&id).unwrap().unwrap();
        assert_eq!(system_from_db.name, system.name);
        std::fs::remove_dir_all(test_db_name).unwrap();
    }

    #[test]
    fn test_add_release() {
        let test_db_name = "test_add_release.db";
        let test_db = DatabaseWithPolo::new(test_db_name);

        let system_id = test_db.add_system(&create_test_system()).unwrap();
        let game_id = test_db.add_game(&create_test_game()).unwrap();
        let collection_file_id = test_db
            .add_collection_file(&create_test_collection_file())
            .unwrap();

        let release = create_test_release(system_id, vec![game_id], vec![collection_file_id]);
        let id = test_db.add_release(&release).unwrap();

        let release_from_db = test_db.get_release(&id).unwrap().unwrap();
        assert_eq!(release_from_db.name, release.name);

        let releases_by_game = test_db.get_releases_with_game(&game_id).unwrap();
        assert_eq!(releases_by_game.len(), 1);
        assert_eq!(releases_by_game[0].name, release.name);

        std::fs::remove_dir_all(test_db_name).unwrap();
    }

    #[test]
    fn test_update_release_with_removed_and_added_game() {
        let test_db_name = "test_update_release.db";
        let test_db = DatabaseWithPolo::new(test_db_name);

        let system_id = test_db.add_system(&create_test_system()).unwrap();
        let game_id_1 = test_db.add_game(&create_test_game()).unwrap();
        let game_id_2 = test_db.add_game(&create_test_game()).unwrap();
        let collection_file_id = test_db
            .add_collection_file(&create_test_collection_file())
            .unwrap();

        let release = create_test_release(system_id, vec![game_id_1], vec![collection_file_id]);

        // add release with game 1

        let id = test_db.add_release(&release).unwrap();

        let release_from_db = test_db.get_release(&id).unwrap().unwrap();
        assert_eq!(release_from_db.games.len(), 1);

        let releases_by_game_1 = test_db.get_releases_by_game(&game_id_1).unwrap().unwrap();

        assert_eq!(releases_by_game_1.release_ids.len(), 1);
        assert_eq!(releases_by_game_1.release_ids[0], id);

        // update release, add game 2

        let mut updated_release = release_from_db.clone();
        updated_release.games.push(game_id_2);
        test_db.update_release(&updated_release).unwrap();

        let release_from_db = test_db.get_release(&id).unwrap().unwrap();
        assert_eq!(release_from_db.games.len(), 2);

        let releases_by_game_2 = test_db.get_releases_by_game(&game_id_2).unwrap().unwrap();
        assert_eq!(releases_by_game_2.release_ids.len(), 1);
        assert_eq!(releases_by_game_2.release_ids[0], id);

        // update release, remove game 1
        let mut updated_release = release_from_db.clone();
        updated_release
            .games
            .retain(|game_id| *game_id != game_id_1);

        test_db.update_release(&updated_release).unwrap();
        let release_from_db = test_db.get_release(&id).unwrap().unwrap();
        assert_eq!(release_from_db.games.len(), 1);

        let releases_by_game_1 = test_db.get_releases_by_game(&game_id_1).unwrap().unwrap();
        assert_eq!(releases_by_game_1.release_ids.len(), 0);

        std::fs::remove_dir_all(test_db_name).unwrap();
    }

    #[test]
    fn test_delete_release_with_files() {
        let test_db_name = "test_delete_release_with_files.db";
        let test_db = DatabaseWithPolo::new(test_db_name);

        let system_id = test_db.add_system(&create_test_system()).unwrap();
        let game_id = test_db.add_game(&create_test_game()).unwrap();
        let collection_file_id = test_db
            .add_collection_file(&create_test_collection_file())
            .unwrap();

        let release = create_test_release(system_id, vec![game_id], vec![collection_file_id]);
        let id = test_db.add_release(&release).unwrap();

        let release_from_db = test_db.get_release(&id).unwrap().unwrap();
        assert_eq!(release_from_db.name, release.name);

        let result = test_db.delete_release(&id);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Release cannot be deleted because it has files"));

        std::fs::remove_dir_all(test_db_name).unwrap();
    }

    #[test]
    fn test_delete_release_without_files() {
        let test_db_name = "test_delete_release_without_files.db";
        let test_db = DatabaseWithPolo::new(test_db_name);

        let system_id = test_db.add_system(&create_test_system()).unwrap();
        let game_id = test_db.add_game(&create_test_game()).unwrap();
        let release = create_test_release(system_id, vec![game_id], vec![]);
        let id = test_db.add_release(&release).unwrap();

        let release_from_db = test_db.get_release(&id).unwrap().unwrap();
        assert_eq!(release_from_db.name, release.name);

        // check that release is in releases_by_game
        let releases_by_game = test_db.get_releases_by_game(&game_id).unwrap().unwrap();
        assert_eq!(releases_by_game.release_ids.first().unwrap(), &id);

        let result = test_db.delete_release(&id);
        assert!(result.is_ok());
        let release_from_db = test_db.get_release(&id).unwrap();
        assert!(release_from_db.is_none());

        let releases_by_game = test_db.get_releases_by_game(&game_id).unwrap();
        assert!(releases_by_game.is_none());

        std::fs::remove_dir_all(test_db_name).unwrap();
    }

    #[test]
    fn test_delete_collection_file_that_is_in_release() {
        let test_db_name = "test_delete_collection_file_that_is_in_release.db";
        let test_db = DatabaseWithPolo::new(test_db_name);

        let system_id = test_db.add_system(&create_test_system()).unwrap();
        let game_id = test_db.add_game(&create_test_game()).unwrap();
        let collection_file_id = test_db
            .add_collection_file(&create_test_collection_file())
            .unwrap();

        let release = create_test_release(system_id, vec![game_id], vec![collection_file_id]);
        let id = test_db.add_release(&release).unwrap();

        let release_from_db = test_db.get_release(&id).unwrap().unwrap();
        assert_eq!(release_from_db.name, release.name);

        let result = test_db.delete_collection_file(&collection_file_id);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Collection file cannot be deleted because it is used in a release"));

        std::fs::remove_dir_all(test_db_name).unwrap();
    }
}
