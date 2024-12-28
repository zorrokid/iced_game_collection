use std::{
    fmt::{self, Display, Formatter},
    path::Path,
};

use bson::{oid::ObjectId, to_bson};
use polodb_core::bson::Bson;
use serde::{Deserialize, Serialize};

use super::model::{GetIdString, HasOid};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CollectionFileType {
    Rom,
    DiskImage,
    TapeImage,
    ScreenShot,
    Manual,
    CoverScan,
}

impl CollectionFileType {
    pub fn directory(&self) -> &str {
        match self {
            CollectionFileType::Rom => "roms",
            CollectionFileType::DiskImage => "disk_images",
            CollectionFileType::TapeImage => "tape_images",
            CollectionFileType::ScreenShot => "screenshots",
            CollectionFileType::Manual => "manuals",
            CollectionFileType::CoverScan => "cover_scans",
        }
    }
}

impl ToString for CollectionFileType {
    fn to_string(&self) -> String {
        match self {
            CollectionFileType::Rom => "Rom".to_string(),
            CollectionFileType::DiskImage => "Disk Image".to_string(),
            CollectionFileType::TapeImage => "Tape Image".to_string(),
            CollectionFileType::ScreenShot => "Screen Shot".to_string(),
            CollectionFileType::Manual => "Manual".to_string(),
            CollectionFileType::CoverScan => "Cover scan".to_string(),
        }
    }
}

impl HasOid for CollectionFile {
    fn id(&self) -> ObjectId {
        self._id.clone().expect("Object id not set")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileInfo {
    pub name: String,
    pub checksum: String,
}

pub trait GetFileExtensions {
    fn get_file_extensions(&self) -> Vec<String>;
}

pub trait GetCollectionFileName {
    fn get_collection_file_name(&self) -> String;
}

impl GetIdString for CollectionFile {
    fn get_id_string(&self) -> String {
        self.id().to_hex()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CollectionFile {
    pub _id: Option<ObjectId>,
    pub original_file_name: String,
    pub is_zip: bool,
    pub files: Option<Vec<FileInfo>>,
    pub collection_file_type: CollectionFileType,
}

impl GetFileExtensions for CollectionFile {
    fn get_file_extensions(&self) -> Vec<String> {
        match &self.files {
            Some(files) => files
                .iter()
                .map(|file| {
                    file.name
                        .split('.')
                        .last()
                        .unwrap()
                        .to_string()
                        .to_lowercase()
                })
                .collect::<Vec<String>>(),
            None => vec![],
        }
    }
}

impl GetCollectionFileName for CollectionFile {
    fn get_collection_file_name(&self) -> String {
        let extension = Path::new(&self.original_file_name).extension();
        if let Some(extension) = extension {
            if let Some(extension) = extension.to_str() {
                return format!(
                    "{}.{}",
                    self.get_id_string(),
                    extension.to_string().to_lowercase()
                );
            }
        }
        self.get_id_string()
    }
}

impl Display for CollectionFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.original_file_name)
    }
}

impl Into<Bson> for CollectionFile {
    fn into(self) -> Bson {
        to_bson(&self).unwrap_or(Bson::Null)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_file_extensions() {
        let collection_file = CollectionFile {
            original_file_name: "game.zip".to_string(),
            _id: Some(ObjectId::new()),
            is_zip: true,
            files: Some(vec![FileInfo {
                name: "game.rom".to_string(),
                checksum: "checksum".to_string(),
            }]),
            collection_file_type: CollectionFileType::Rom,
        };

        let extensions = collection_file.get_file_extensions();
        assert_eq!(extensions, vec!["rom".to_string()]);
    }

    #[test]
    fn test_get_collection_file_name() {
        let collection_file = CollectionFile {
            original_file_name: "game.zip".to_string(),
            _id: Some(ObjectId::new()),
            is_zip: true,
            files: None,
            collection_file_type: CollectionFileType::Rom,
        };

        let file_name = collection_file.get_collection_file_name();
        assert_eq!(
            file_name,
            format!("{}.zip", &collection_file.get_id_string())
        );
    }
}
