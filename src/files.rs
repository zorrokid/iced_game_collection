use crate::error::Error;
use crate::model::{FileInfo, FolderType, PickedFile};
use async_std::fs::{copy, remove_file, File};
use async_std::path::Path as AsyncPath;
use async_std::prelude::*;
use sha1::{Digest, Sha1};
use std::io::{Cursor, Read};
use std::path::Path;
use std::path::PathBuf as StdPathBuf;
use zip::read::ZipArchive;

pub async fn pick_folder(folder_type: FolderType) -> Result<(StdPathBuf, FolderType), Error> {
    let file_handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a folder")
        .pick_folder()
        .await
        .ok_or(Error::DialogClosed)?;
    Ok((file_handle.path().to_owned(), folder_type))
}

pub async fn pick_file(source_path: String, destination_path: String) -> Result<PickedFile, Error> {
    let file_handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a file")
        .set_directory(source_path)
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    let is_zip = is_zip_file(file_handle.path()).await?;

    let files = if is_zip {
        Some(read_zip_file(file_handle.path().to_str().unwrap()).await?)
    } else {
        None
    };

    let file_name = file_handle
        .path()
        .file_name()
        .ok_or(Error::IoError("file name not available".to_string()))?
        .to_owned();

    let path = AsyncPath::new(&destination_path).join(file_name.clone());

    copy(file_handle.path(), &path)
        .await
        .map_err(|e| Error::IoError(format!("Failed to copy file: {}", e)))?;

    Ok(PickedFile {
        file_name,
        is_zip,
        files,
    })
}

pub async fn delete_files(file_names: Vec<String>, path: String, id: i32) -> Result<i32, Error> {
    for file_name in file_names {
        let file_path = Path::new(&path).join(file_name);
        remove_file(file_path)
            .await
            .map_err(|e| Error::IoError(format!("Failed to delete file: {}", e)))?;
    }
    Ok(id)
}

pub async fn read_zip_file(file_path: &str) -> Result<Vec<FileInfo>, Error> {
    let file = File::open(file_path)
        .await
        .map_err(|e| Error::IoError("Failed opening file.".to_string()))?;
    let mut buffer = Vec::new();
    file.take(10 * 1024 * 1024) // Read up to 10 MB
        .read_to_end(&mut buffer)
        .await
        .map_err(|e| Error::IoError("Failed reaind contents of file.".to_string()))?;

    let reader = Cursor::new(buffer);
    let mut zip = ZipArchive::new(reader).map_err(|e| {
        Error::IoError("Failed creating a Zip archive from the buffer.".to_string())
    })?;

    let mut file_infos = Vec::new();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).map_err(|e| {
            Error::IoError(format!("Failed reading file in index {} in zip file.", i))
        })?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).map_err(|e| {
            Error::IoError(format!(
                "Failed reading file {} from Zip archive.",
                file.name()
            ))
        })?;

        let mut hasher = Sha1::new();
        hasher.update(&contents);
        let checksum = format!("{:x}", hasher.finalize());

        file_infos.push(FileInfo {
            name: file.name().to_string(),
            checksum,
        });
    }

    Ok(file_infos)
}

pub async fn is_zip_file(file_path: &Path) -> Result<bool, Error> {
    const ZIP_MAGIC_NUMBER: [u8; 4] = [0x50, 0x4B, 0x03, 0x04];

    let mut file = File::open(file_path)
        .await
        .map_err(|_| Error::IoError(format!("Failed opening file {:?}.", file_path.file_name())))?;
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer).await.map_err(|_| {
        Error::IoError(format!(
            "Failed reading from file {:?}.",
            file_path.file_name()
        ))
    })?;

    Ok(buffer == ZIP_MAGIC_NUMBER)
}
