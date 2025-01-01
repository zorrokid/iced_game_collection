use bson::oid::ObjectId;

use crate::{
    error::Error,
    model::{
        collection_file::CollectionFile,
        model::{Game, Release, System},
    },
};

pub trait ReleaseReadRepository {
    fn get_release(&self, id: &ObjectId) -> Result<Option<Release>, Error>;
    fn get_releases_with_game(&self, id: &ObjectId) -> Result<Vec<Release>, Error>;
}

pub trait GamesReadRepository {
    fn get_games(&self, ids: &Vec<ObjectId>) -> Result<Vec<Game>, Error>;
    fn get_all_games(&self) -> Result<Vec<Game>, Error>;
    fn is_game_in_release(&self, game_id: &ObjectId) -> Result<bool, Error>;
}

pub trait CollectionFilesReadRepository {
    fn get_collection_files(&self, ids: &Vec<ObjectId>) -> Result<Vec<CollectionFile>, Error>;
}

pub trait SystemReadRepository {
    fn get_system(&self, id: &ObjectId) -> Result<Option<System>, Error>;
    fn get_all_systems(&self) -> Result<Vec<System>, Error>;
    fn is_system_in_release(&self, system_id: &ObjectId) -> Result<bool, Error>;
}
