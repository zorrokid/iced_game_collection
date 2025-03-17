use crate::model::{
    collection_file::CollectionFile,
    model::{SoftwareTitle, System},
};

pub struct ReleaseViewModel {
    pub id: Option<i64>,
    pub name: String,
    pub system: Vec<System>,
    pub files: Vec<CollectionFile>,
    pub games: Vec<SoftwareTitle>,
}
