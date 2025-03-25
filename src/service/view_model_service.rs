use std::sync::Arc;

use crate::{
    database::{
        database_error::DatabaseError, release_repository::ReleaseReadRepository,
        repository_manager::RepositoryManager, setting_repository::SettingReadRepository,
        software_title_repository::SoftwareTitleReadRepository,
        system_repository::SystemReadRepository as _,
    },
    view_model::{
        list_models::{ReleaseListModel, SoftwareTitleListModel, SystemListModel},
        settings::Settings,
    },
};

#[derive(Debug, Clone)]
pub struct ViewModelService {
    repository_manager: Arc<RepositoryManager>,
}

impl ViewModelService {
    pub fn new(repository_manager: Arc<RepositoryManager>) -> Self {
        Self { repository_manager }
    }

    pub async fn get_software_title_list_models(
        &self,
    ) -> Result<Vec<SoftwareTitleListModel>, DatabaseError> {
        let software_titles = self
            .repository_manager
            .software_titles()
            .get_all_software_titles()
            .await?;

        let mut list_models: Vec<SoftwareTitleListModel> = software_titles
            .iter()
            .map(SoftwareTitleListModel::from)
            .collect();

        for software_title in list_models.iter_mut() {
            software_title.can_delete = !self
                .repository_manager
                .software_titles()
                .is_software_title_in_release(software_title.id)
                .await?;
        }

        Ok(list_models)
    }

    pub async fn get_system_list_models(&self) -> Result<Vec<SystemListModel>, DatabaseError> {
        let systems = self.repository_manager.systems().get_systems().await?;

        let mut list_models: Vec<SystemListModel> =
            systems.iter().map(SystemListModel::from).collect();

        for system in list_models.iter_mut() {
            system.can_delete = !self
                .repository_manager
                .systems()
                .is_system_in_use(system.id)
                .await?;
        }

        Ok(list_models)
    }

    pub async fn get_release_list_models(
        &self,
        software_title_id: i64,
    ) -> Result<Vec<ReleaseListModel>, DatabaseError> {
        let releases = self
            .repository_manager
            .releases()
            .get_releases_with_software_title(software_title_id)
            .await?;

        let mut list_models: Vec<ReleaseListModel> = Vec::new();

        for release in releases {
            let system = self
                .repository_manager
                .systems()
                .get_systems_for_release(release.id)
                .await?;

            let can_delete = !self
                .repository_manager
                .releases()
                .has_release_files(release.id)
                .await?;

            let system_names = system
                .iter()
                .map(|s| s.name.clone())
                .collect::<Vec<String>>();

            list_models.push(ReleaseListModel {
                id: release.id,
                name: release.name.clone(),
                system_name: system_names.join(", "),
                can_delete,
            });
        }

        Ok(list_models)
    }

    pub async fn get_settings(&self) -> Result<Settings, DatabaseError> {
        let settings_map = self.repository_manager.settings().get_settings().await?;
        Ok(Settings::from(settings_map))
    }
}
