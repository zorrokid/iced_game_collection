use std::{
    fmt::{self, Display, Formatter},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::impl_has_oid;

use super::model::{HasIdField, HasOid};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CollectionFileType {
    Rom,
    DiskImage,
    TapeImage,
    Screenshot,
    Manual,
    CoverScan,
    MemorySnapshot,
}

impl Into<i64> for CollectionFileType {
    fn into(self) -> i64 {
        match self {
            CollectionFileType::Rom => 1,
            CollectionFileType::DiskImage => 2,
            CollectionFileType::TapeImage => 3,
            CollectionFileType::Screenshot => 4,
            CollectionFileType::Manual => 5,
            CollectionFileType::CoverScan => 6,
            CollectionFileType::MemorySnapshot => 7,
        }
    }
}

impl From<i64> for CollectionFileType {
    fn from(value: i64) -> Self {
        match value {
            1 => CollectionFileType::Rom,
            2 => CollectionFileType::DiskImage,
            3 => CollectionFileType::TapeImage,
            4 => CollectionFileType::Screenshot,
            5 => CollectionFileType::Manual,
            6 => CollectionFileType::CoverScan,
            7 => CollectionFileType::MemorySnapshot,
            _ => panic!("Invalid CollectionFileType value"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ArchiveType {
    Zip,
}

impl Into<i64> for ArchiveType {
    fn into(self) -> i64 {
        match self {
            ArchiveType::Zip => 1,
        }
    }
}

impl From<i64> for ArchiveType {
    fn from(value: i64) -> Self {
        match value {
            1 => ArchiveType::Zip,
            _ => panic!("Invalid ArchiveType value"),
        }
    }
}

impl CollectionFileType {
    pub fn directory(&self) -> &str {
        match self {
            CollectionFileType::Rom => "roms",
            CollectionFileType::DiskImage => "disk_images",
            CollectionFileType::TapeImage => "tape_images",
            CollectionFileType::Screenshot => "screenshots",
            CollectionFileType::Manual => "manuals",
            CollectionFileType::CoverScan => "cover_scans",
            CollectionFileType::MemorySnapshot => "memory_snapshots",
        }
    }
}

impl ToString for CollectionFileType {
    fn to_string(&self) -> String {
        match self {
            CollectionFileType::Rom => "Rom".to_string(),
            CollectionFileType::DiskImage => "Disk Image".to_string(),
            CollectionFileType::TapeImage => "Tape Image".to_string(),
            CollectionFileType::Screenshot => "Screenshot".to_string(),
            CollectionFileType::Manual => "Manual".to_string(),
            CollectionFileType::CoverScan => "Cover Scan".to_string(),
            CollectionFileType::MemorySnapshot => "Memory Snapshot".to_string(),
        }
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

pub trait GetFileExtension {
    fn get_file_extension(&self) -> String;
}

pub trait GetCollectionFileName {
    fn get_collection_file_name(&self) -> String;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CollectionFile {
    pub id: i64,
    pub original_file_name: String,
    pub is_archive: bool,
    pub archive_type: ArchiveType,
    //pub files: Option<Vec<FileInfo>>,
    pub file_type: CollectionFileType,
}

impl_has_oid!(CollectionFile);

fn get_file_extension(file_name: &str) -> String {
    file_name
        .split('.')
        .last()
        .unwrap()
        .to_string()
        .to_lowercase()
}

impl GetFileExtensions for CollectionFile {
    fn get_file_extensions(&self) -> Vec<String> {
        match &self.files {
            Some(files) => files
                .iter()
                .map(|file| get_file_extension(file.name.as_str()))
                .collect::<Vec<String>>(),
            None => vec![],
        }
    }
}

impl GetFileExtension for CollectionFile {
    fn get_file_extension(&self) -> String {
        get_file_extension(self.original_file_name.as_str())
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_file_extensions() {
        let collection_file = CollectionFile {
            original_file_name: "game.zip".to_string(),
            id: Some(1),
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
            id: Some(1),
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
