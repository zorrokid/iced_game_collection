use std::fmt::{self, Display, Formatter};

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
    pub id: i64,
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
            id: system.id,
            name: system.name.clone(),
            can_delete: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReleaseListModel {
    pub id: i64,
    pub name: String,
    pub system_name: String,
    pub can_delete: bool,
}
