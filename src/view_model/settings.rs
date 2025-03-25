use std::{collections::HashMap, path::PathBuf};

use crate::model::settings::SettingName;

#[derive(Debug, Clone)]
pub struct Settings {
    // TODO: maybe wrap into an Option, so that we can check if it's set
    pub collection_root_dir: Option<PathBuf>,
}

impl From<HashMap<String, String>> for Settings {
    fn from(map: HashMap<String, String>) -> Self {
        Self {
            collection_root_dir: Some(PathBuf::from(
                map.get(SettingName::CollectionRootDir.as_str())
                    .unwrap_or(None),
            )),
        }
    }
}
