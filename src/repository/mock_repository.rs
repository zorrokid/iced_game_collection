use super::repository::{
    CollectionFilesReadRepository, GamesReadRepository, ReleaseReadRepository, SystemReadRepository,
};
use crate::{
    error::Error,
    model::{
        collection_file::CollectionFile,
        model::{Game, Release, System},
    },
};
use bson::oid::ObjectId;
use std::collections::HashMap;

pub struct MockRepository {
    pub releases: HashMap<ObjectId, Release>,
    pub games: HashMap<ObjectId, Game>,
    pub collection_files: HashMap<ObjectId, CollectionFile>,
    pub systems: HashMap<ObjectId, System>,
}

impl ReleaseReadRepository for MockRepository {
    fn get_release(&self, id: &ObjectId) -> Result<Option<Release>, Error> {
        Ok(self.releases.get(id).cloned())
    }
}

impl GamesReadRepository for MockRepository {
    fn get_games(&self, ids: &Vec<ObjectId>) -> Result<Vec<Game>, Error> {
        Ok(ids
            .iter()
            .filter_map(|id| self.games.get(id).cloned())
            .collect())
    }
}

impl CollectionFilesReadRepository for MockRepository {
    fn get_collection_files(&self, ids: &Vec<ObjectId>) -> Result<Vec<CollectionFile>, Error> {
        Ok(ids
            .iter()
            .filter_map(|id| self.collection_files.get(id).cloned())
            .collect())
    }
}

impl SystemReadRepository for MockRepository {
    fn get_system(&self, id: &ObjectId) -> Result<Option<System>, Error> {
        Ok(self.systems.get(id).cloned())
    }
}
