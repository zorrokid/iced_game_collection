pub enum SettingName {
    CollectionRootDir,
}

impl SettingName {
    pub fn as_str(&self) -> &'static str {
        match self {
            SettingName::CollectionRootDir => "collection_root_dir",
        }
    }
}
