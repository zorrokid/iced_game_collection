/*use crate::{
    error::Error,
    model::{
        collection_file::CollectionFile,
        model::{HasOid, Release, ReleasesByGame, SoftwareTitle, System},
    },
};
use bson::oid::ObjectId;
use std::collections::HashMap;

use super::{database_error::DatabaseError, release_repository::ReleaseReadRepository};

pub struct MockRepository {
    pub releases: HashMap<ObjectId, Release>,
    pub games: HashMap<ObjectId, SoftwareTitle>,
    pub collection_files: HashMap<ObjectId, CollectionFile>,
    pub systems: HashMap<ObjectId, System>,
}

impl ReleaseReadRepository for MockRepository {
    async fn get_release(&self, id: i64) -> Result<Release, DatabaseError> {
        Ok(self.releases.get(id).cloned())
    }
    async fn get_releases_with_software_title(
        &self,
        id: i64,
    ) -> Result<Vec<Release>, DatabaseError> {
        Ok(self
            .releases
            .values()
            .filter(|release| release.games.contains(id))
            .cloned()
            .collect())
    }
}

impl GamesReadRepository for MockRepository {
    fn get_games(&self, ids: &Vec<ObjectId>) -> Result<Vec<SoftwareTitle>, DatabaseError> {
        Ok(ids
            .iter()
            .filter_map(|id| self.games.get(id).cloned())
            .collect())
    }
    fn get_all_games(&self) -> Result<Vec<SoftwareTitle>, DatabaseError> {
        Ok(self.games.values().cloned().collect())
    }
    fn is_game_in_release(&self, game_id: &ObjectId) -> Result<bool, DatabaseError> {
        Ok(self
            .releases
            .values()
            .any(|release| release.games.contains(game_id)))
    }
    fn get_releases_by_game(
        &self,
        game_id: &ObjectId,
    ) -> Result<Option<ReleasesByGame>, DatabaseError> {
        Ok(Some(ReleasesByGame {
            _id: *game_id,
            release_ids: self
                .releases
                .values()
                .filter(|release| release.games.contains(game_id))
                .map(|release| release.id())
                .collect(),
        }))
    }

    fn get_game(&self, id: &ObjectId) -> Result<Option<SoftwareTitle>, Error> {
        Ok(self.games.get(id).cloned())
    }
}

impl CollectionFilesReadRepository for MockRepository {
    fn get_collection_files(
        &self,
        ids: &Vec<ObjectId>,
    ) -> Result<Vec<CollectionFile>, DatabaseError> {
        Ok(ids
            .iter()
            .filter_map(|id| self.collection_files.get(id).cloned())
            .collect())
    }
    fn is_collection_file_in_release(
        &self,
        collection_file_id: &ObjectId,
    ) -> Result<bool, DatabaseError> {
        Ok(self
            .releases
            .values()
            .any(|release| release.files.contains(collection_file_id)))
    }
}

impl SystemReadRepository for MockRepository {
    fn get_system(&self, id: &ObjectId) -> Result<Option<System>, DatabaseError> {
        Ok(self.systems.get(id).cloned())
    }
    fn is_system_in_release(&self, system_id: &ObjectId) -> Result<bool, DatabaseError> {
        Ok(self
            .releases
            .values()
            .any(|release| release.system_id == Some(*system_id)))
    }
    fn get_systems(&self) -> Result<Vec<System>, DatabaseError> {
        Ok(self.systems.values().cloned().collect())
    }
}*/
