use std::path::{Path, PathBuf};

use crate::{
    error::Error,
    model::{CollectionFile, Settings, System},
};

use super::file_path_builder::FilePathBuilder;
use image;

pub fn get_thumbnail_path(
    collection_file: &CollectionFile,
    settings: &Settings,
    system: &System,
) -> Result<PathBuf, Error> {
    let thumbnail_directory = Path::new(&settings.collection_root_dir).join("thumbnails");
    let file_path_builder = FilePathBuilder::new(settings.collection_root_dir.clone());
    let file_path = file_path_builder.build_file_path(system, collection_file);

    let thumbnail_path = thumbnail_directory
        .join(collection_file.id.clone())
        .with_extension("png");

    if thumbnail_path.exists() {
        Ok(thumbnail_path)
    } else {
        let image = image::open(&file_path).map_err(|err| {
            Error::IoError(format!(
                "Failed opening image {} with error: {}",
                file_path.display(),
                &err
            ))
        })?;
        let thumbnail = image.thumbnail(100, 100);
        std::fs::create_dir_all(&thumbnail_directory).map_err(|_| {
            Error::IoError(format!(
                "Failed creating directory: {}",
                &thumbnail_directory.display()
            ))
        })?;

        thumbnail.save(&thumbnail_path).map_err(|err| {
            Error::IoError(format!(
                "Failed saving thumbnail to {} with error: {}",
                thumbnail_path.display(),
                &err
            ))
        })?;
        Ok(thumbnail_path)
    }
}
