use bson::oid::ObjectId;

use crate::{
    error::Error,
    model::{
        collection_file::CollectionFile,
        model::{Franchise, Game, Release, ReleasesByFile, ReleasesByGame, System},
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
    fn get_releases_by_game(&self, game_id: &ObjectId) -> Result<Option<ReleasesByGame>, Error>;
}

pub trait FranchisReadRepository {
    fn get_all_franchises(&self) -> Result<Vec<Franchise>, Error>;
    fn add_franchise(&self, name: &str) -> Result<ObjectId, Error>;
}

pub trait CollectionFilesReadRepository {
    fn get_collection_files(&self, ids: &Vec<ObjectId>) -> Result<Vec<CollectionFile>, Error>;
    fn is_collection_file_in_release(&self, collection_file_id: &ObjectId) -> Result<bool, Error>;
    fn get_releases_by_file(
        &self,
        collection_file_id: &ObjectId,
    ) -> Result<Option<ReleasesByFile>, Error>;
}

pub trait SystemReadRepository {
    fn get_system(&self, id: &ObjectId) -> Result<Option<System>, Error>;
    fn get_all_systems(&self) -> Result<Vec<System>, Error>;
    fn is_system_in_release(&self, system_id: &ObjectId) -> Result<bool, Error>;
}

pub trait ReleaseWriteRepository {
    fn update_release(&self, release: &Release) -> Result<(), Error>;
    fn add_release(&self, release: &Release) -> Result<ObjectId, Error>;
}
