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
}

pub trait GamesReadRepository {
    fn get_games(&self, ids: &Vec<ObjectId>) -> Result<Vec<Game>, Error>;
}

pub trait CollectionFilesReadRepository {
    fn get_collection_files(&self, ids: &Vec<ObjectId>) -> Result<Vec<CollectionFile>, Error>;
}

pub trait SystemReadRepository {
    fn get_system(&self, id: &ObjectId) -> Result<Option<System>, Error>;
}
