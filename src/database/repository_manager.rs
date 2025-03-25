use std::sync::Arc;

use sqlx::{Pool, Sqlite};

use super::{
    collection_file_repository::CollectionFileRepository, emulator_repository::EmulatorRepository,
    franchise_repository::FranchiseRepository, release_repository::ReleaseRepository,
    setting_repository::SettingRepository, software_title_repository::SoftwareTitleRepository,
    system_repository::SystemRepository,
};

#[derive(Debug, Clone)]
pub struct RepositoryManager {
    pool: Arc<Pool<Sqlite>>,
    pub system_repository: SystemRepository,
    pub software_title_repository: SoftwareTitleRepository,
    pub franchise_repository: FranchiseRepository,
    pub release_repository: ReleaseRepository,
    pub emulator_repository: EmulatorRepository,
    pub collection_file_repository: CollectionFileRepository,
    pub setting_repository: SettingRepository,
}

impl RepositoryManager {
    pub fn new(pool: Arc<Pool<Sqlite>>) -> Self {
        let system_repository = SystemRepository::new(pool.clone());
        let software_title_repository = SoftwareTitleRepository::new(pool.clone());
        let franchise_repository = FranchiseRepository::new(pool.clone());
        let release_repository = ReleaseRepository::new(pool.clone());
        let emulator_repository = EmulatorRepository::new(pool.clone());
        let collection_file_repository = CollectionFileRepository::new(pool.clone());
        let setting_repository = SettingRepository::new(pool.clone());

        Self {
            pool,
            system_repository,
            software_title_repository,
            franchise_repository,
            release_repository,
            emulator_repository,
            collection_file_repository,
            setting_repository,
        }
    }

    pub fn settings(&self) -> &SettingRepository {
        &self.setting_repository
    }

    pub fn systems(&self) -> &SystemRepository {
        &self.system_repository
    }

    pub fn software_titles(&self) -> &SoftwareTitleRepository {
        &self.software_title_repository
    }

    pub fn franchises(&self) -> &FranchiseRepository {
        &self.franchise_repository
    }

    pub fn releases(&self) -> &ReleaseRepository {
        &self.release_repository
    }

    pub fn emulators(&self) -> &EmulatorRepository {
        &self.emulator_repository
    }

    pub fn collection_files(&self) -> &CollectionFileRepository {
        &self.collection_file_repository
    }
}
