use crate::error::Error;
use crate::model::FolderType;
use async_std::fs;
use async_std::path::{Path, PathBuf};
use std::path::PathBuf as StdPathBuf;

pub async fn pick_folder(folder_type: FolderType) -> Result<(StdPathBuf, FolderType), Error> {
    let file_handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a folder")
        .pick_folder()
        .await
        .ok_or(Error::DialogClosed)?;
    Ok((file_handle.path().to_owned(), folder_type))
}

pub async fn pick_file(source_path: String, destination_path: String) -> Result<PathBuf, Error> {
    let file_handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a file")
        .set_directory(source_path)
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    let file_name = file_handle
        .path()
        .file_name()
        .ok_or(Error::IoError("file name not available".to_string()))?
        .to_owned();

    let destination_path = Path::new(&destination_path).join(file_name);

    fs::copy(file_handle.path(), &destination_path)
        .await
        .map_err(|e| Error::IoError(format!("Failed to copy file: {}", e)))?;

    Ok(destination_path)
}

pub async fn delete_files(file_names: Vec<String>, path: String, id: i32) -> Result<i32, Error> {
    for file_name in file_names {
        let file_path = Path::new(&path).join(file_name);
        fs::remove_file(file_path)
            .await
            .map_err(|e| Error::IoError(format!("Failed to delete file: {}", e)))?;
    }
    Ok(id)
}
