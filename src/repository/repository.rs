use bson::oid::ObjectId;

use crate::{
    error::Error,
    model::{
        collection_file::CollectionFile,
        model::{Emulator, Franchise, Game, Release, ReleasesByGame, Settings, System},
    },
};

pub trait ReleaseReadRepository {
    fn get_release(&self, id: &ObjectId) -> Result<Option<Release>, Error>;
    fn get_releases_with_game(&self, id: &ObjectId) -> Result<Vec<Release>, Error>;
}

pub trait GamesReadRepository {
    fn get_game(&self, id: &ObjectId) -> Result<Option<Game>, Error>;
    fn get_games(&self, ids: &Vec<ObjectId>) -> Result<Vec<Game>, Error>;
    fn get_all_games(&self) -> Result<Vec<Game>, Error>;
    fn is_game_in_release(&self, game_id: &ObjectId) -> Result<bool, Error>;
    fn get_releases_by_game(&self, game_id: &ObjectId) -> Result<Option<ReleasesByGame>, Error>;
}

pub trait GamesWriteRepository {
    fn add_game(&self, game: &Game) -> Result<ObjectId, Error>;
    fn update_game(&self, game: &Game) -> Result<ObjectId, Error>;
    fn delete_game(&self, id: &ObjectId) -> Result<(), Error>;
}

pub trait FranchisReadRepository {
    fn get_all_franchises(&self) -> Result<Vec<Franchise>, Error>;
    fn add_franchise(&self, name: &str) -> Result<ObjectId, Error>;
}

pub trait CollectionFilesReadRepository {
    fn get_collection_files(&self, ids: &Vec<ObjectId>) -> Result<Vec<CollectionFile>, Error>;
    fn is_collection_file_in_release(&self, collection_file_id: &ObjectId) -> Result<bool, Error>;
}

pub trait CollectionFileWriteRepository {
    fn add_collection_file(&self, collection_file: &CollectionFile) -> Result<ObjectId, Error>;
    fn delete_collection_file(&self, id: &ObjectId) -> Result<(), Error>;
}

pub trait SystemReadRepository {
    fn get_system(&self, id: &ObjectId) -> Result<Option<System>, Error>;
    fn get_systems(&self) -> Result<Vec<System>, Error>;
    fn is_system_in_release(&self, system_id: &ObjectId) -> Result<bool, Error>;
}

pub trait SystemWriteRepository {
    fn add_system(&self, system: &System) -> Result<ObjectId, Error>;
    fn update_system(&self, system: &System) -> Result<ObjectId, Error>;
    fn delete_system(&self, id: &ObjectId) -> Result<(), Error>;
}

pub trait ReleaseWriteRepository {
    fn update_release(&self, release: &Release) -> Result<ObjectId, Error>;
    fn add_release(&self, release: &Release) -> Result<ObjectId, Error>;
    fn delete_release(&self, id: &ObjectId) -> Result<(), Error>;
}

pub trait EmulatorReadRepository {
    fn get_emulators(&self) -> Result<Vec<Emulator>, Error>;
    fn get_emulator(&self, id: &ObjectId) -> Result<Option<Emulator>, Error>;
}

pub trait EmulatorWriteRepository {
    fn add_emulator(&self, name: &Emulator) -> Result<ObjectId, Error>;
    fn delete_emulator(&self, id: &ObjectId) -> Result<(), Error>;
    fn update_emulator(&self, emulator: &Emulator) -> Result<ObjectId, Error>;
}

pub trait SettingsReadRepository {
    fn get_settings(&self) -> Result<Settings, Error>;
}

pub trait SettingsWriteRepository {
    fn add_or_update_settings(&self, settings: &Settings) -> Result<String, Error>;
}
