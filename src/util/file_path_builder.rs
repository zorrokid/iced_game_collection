use crate::{
    error::Error,
    files::get_file_extension,
    model::{
        collection_file::{CollectionFile, CollectionFileType},
        model::{GetIdString, HasOid, System},
    },
};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FilePathBuilder {
    pub collection_root_dir: String,
}

impl FilePathBuilder {
    pub fn new(collection_root_dir: String) -> Self {
        Self {
            collection_root_dir,
        }
    }

    pub fn build_file_path(
        &self,
        system: &System,
        collection_file: &CollectionFile,
    ) -> Result<PathBuf, Error> {
        let mut path = PathBuf::from(&self.collection_root_dir);

        let extension = get_file_extension(Path::new(&collection_file.original_file_name))?;
        path.push(&system.id().to_string());
        path.push(&collection_file.collection_file_type.directory());
        path.push(&collection_file.get_id_string());
        Ok(path.with_extension(extension))
    }

    pub fn build_target_directory(
        &self,
        system: &System,
        file_type: &CollectionFileType,
    ) -> PathBuf {
        let mut path = PathBuf::from(&self.collection_root_dir);
        path.push(&system.id().to_hex());
        path.push(&file_type.directory());
        path
    }
}

#[cfg(test)]
mod tests {
    use bson::oid::ObjectId;

    use super::*;
    use crate::model::{
        collection_file::{CollectionFileType, FileInfo},
        model::GetIdString,
    };
    use std::path::PathBuf;

    #[test]
    fn test_build_file_path() {
        let collection_root_dir = "/home/user/collection".to_string();
        let file_path_builder = FilePathBuilder::new(collection_root_dir);

        let system = System {
            _id: Some(ObjectId::new()),
            name: "System".to_string(),
            notes: None,
        };

        let collection_file = CollectionFile {
            _id: Some(ObjectId::new()),
            original_file_name: "file.zip".to_string(),
            is_zip: true,
            files: Some(vec![FileInfo {
                name: "file1".to_string(),
                checksum: "checksum".to_string(),
            }]),
            collection_file_type: CollectionFileType::DiskImage,
        };

        let result = file_path_builder.build_file_path(&system, &collection_file);
        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(
            path,
            PathBuf::from(format!(
                "/home/user/collection/{}/disk_images/{}.zip",
                system.get_id_string(),
                collection_file.get_id_string()
            ))
        );
    }

    #[test]
    fn test_build_target_directory() {
        let collection_root_dir = "/home/user/collection".to_string();
        let file_path_builder = FilePathBuilder::new(collection_root_dir);

        let system = System {
            _id: Some(ObjectId::new()),
            name: "System".to_string(),
            notes: None,
        };

        let file_type = CollectionFileType::DiskImage;

        let path = file_path_builder.build_target_directory(&system, &file_type);
        assert_eq!(
            path,
            PathBuf::from(format!(
                "/home/user/collection/{}/disk_images",
                system.id().to_hex()
            ))
        );
    }
}
