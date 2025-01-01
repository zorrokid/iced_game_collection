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
    fn get_releases_with_game(&self, id: &ObjectId) -> Result<Vec<Release>, Error> {
        Ok(self
            .releases
            .values()
            .filter(|release| release.games.contains(id))
            .cloned()
            .collect())
    }
}

impl GamesReadRepository for MockRepository {
    fn get_games(&self, ids: &Vec<ObjectId>) -> Result<Vec<Game>, Error> {
        Ok(ids
            .iter()
            .filter_map(|id| self.games.get(id).cloned())
            .collect())
    }
    fn get_all_games(&self) -> Result<Vec<Game>, Error> {
        Ok(self.games.values().cloned().collect())
    }
    fn is_game_in_release(&self, game_id: &ObjectId) -> Result<bool, Error> {
        Ok(self
            .releases
            .values()
            .any(|release| release.games.contains(game_id)))
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
    fn is_system_in_release(&self, system_id: &ObjectId) -> Result<bool, Error> {
        Ok(self
            .releases
            .values()
            .any(|release| release.system_id == Some(*system_id)))
    }
    fn get_all_systems(&self) -> Result<Vec<System>, Error> {
        Ok(self.systems.values().cloned().collect())
    }
}
