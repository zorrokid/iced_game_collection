use crate::model::model::{SoftwareTitle, System};

#[derive(Debug, Clone)]
pub struct SoftwareTitleListModel {
    pub id: i64,
    pub name: String,
    pub can_delete: bool,
}

impl From<&SoftwareTitle> for SoftwareTitleListModel {
    fn from(software_title: &SoftwareTitle) -> Self {
        SoftwareTitleListModel {
            id: software_title.id,
            name: software_title.name.clone(),
            can_delete: false,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct SystemListModel {
    pub id: ObjectId,
    pub name: String,
    pub can_delete: bool,
}

impl Display for SystemListModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<&System> for SystemListModel {
    fn from(system: &System) -> Self {
        SystemListModel {
            id: system.id(),
            name: system.name.clone(),
            can_delete: false,
        }
    }
}

/*
pub fn get_systems_in_list_model<R>(repository: &R) -> Result<Vec<SystemListModel>, Error>
where
    R: SystemReadRepository,
{
    let systems = repository.get_systems()?;
    let mut list_models: Vec<SystemListModel> = systems.iter().map(SystemListModel::from).collect();
    for system in &mut list_models {
        system.can_delete = !repository.is_system_in_release(&system.id)?;
    }
    Ok(list_models)
}

#[derive(Debug, Clone)]
pub struct ReleaseListModel {
    pub id: ObjectId,
    pub name: String,
    pub system_name: String,
    pub can_delete: bool,
}

pub fn get_releases_in_list_model<R>(
    repository: &R,
    game_id: &ObjectId,
) -> Result<Vec<ReleaseListModel>, Error>
where
    R: ReleaseReadRepository + SystemReadRepository,
{
    let releases = repository.get_releases_with_software_title(game_id)?;
    let mut list_models: Vec<ReleaseListModel> = Vec::new();
    for release in releases {
        let system_id = &release.system_id.expect("Expected system_id");
        let system_name = repository
            .get_system(system_id)?
            .expect("System not found")
            .name;
        let can_delete = release.files.is_empty();
        list_models.push(ReleaseListModel {
            id: release.id(),
            name: release.name.clone(),
            system_name,
            can_delete,
        });
    }
    Ok(list_models)
}*/
