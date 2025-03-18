use std::sync::Arc;

use crate::{
    database::{
        database_error::DatabaseError, repository_manager::RepositoryManager,
        software_title_repository::SoftwareTitleReadRepository,
        system_repository::SystemReadRepository as _,
    },
    view_model::list_models::{SoftwareTitleListModel, SystemListModel},
};

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
}
