use crate::error::Error;
use crate::model::collection_file::{CollectionFile, FileInfo, GetCollectionFileName};
use async_std::fs::{copy as async_copy, create_dir_all, remove_file, File as AsyncFile};
use async_std::path::{Path as AsyncPath, PathBuf};
use async_std::prelude::*;
use sha1::{Digest, Sha1};
use std::fs::{copy, File};
use std::io::Write;
use std::io::{Cursor, Read};
use std::path::{Path as SyncPath, PathBuf as SyncPathBuf};
use zip::read::ZipArchive;

#[derive(Debug, Clone)]
pub struct PickedFile {
    pub file_name: String,
    pub is_zip: bool,
    pub files: Option<Vec<FileInfo>>,
    pub file_path: PathBuf,
    pub extension: String,
}

pub async fn pick_folder() -> Result<SyncPathBuf, Error> {
    let file_handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a folder")
        .pick_folder()
        .await
        .ok_or(Error::DialogClosed)?;
    Ok(file_handle.path().to_owned())
}

pub async fn pick_file() -> Result<PickedFile, Error> {
    let picked_file_handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a file")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    let picked_file_path = picked_file_handle.path();

    println!("picked_file_path: {:?}", picked_file_path);

    let is_zip = is_zip_file(AsyncPath::new(picked_file_path)).await?;

    println!("is_zip: {:?}", is_zip);

    let files_in_zip = if is_zip {
        Some(read_zip_file(picked_file_path).await?)
    } else {
        None
    };

    let picked_file_name = get_file_name(picked_file_path)?;
    let picked_file_extension = get_file_extension(picked_file_path)?;

    println!("picked_file_name: {:?}", picked_file_name);

    Ok(PickedFile {
        file_name: picked_file_name,
        is_zip,
        files: files_in_zip,
        file_path: PathBuf::from(picked_file_path),
        extension: picked_file_extension,
    })
}

pub fn get_file_name(path: &SyncPath) -> Result<String, Error> {
    let file_name = path
        .file_name()
        .ok_or(Error::IoError("file name not available".to_string()))?
        .to_owned()
        .into_string()
        .map_err(|_| {
            Error::IoError(format!(
                "Failed to get file name (invalid unicode data in file name)"
            ))
        })?;
    Ok(file_name)
}

pub fn get_file_extension(path: &SyncPath) -> Result<String, Error> {
    let extension = path
        .extension()
        .ok_or(Error::IoError("file extension not available".to_string()))?
        .to_owned()
        .into_string()
        .map_err(|_| {
            Error::IoError(format!(
                "Failed to get file extension (invalid unicode data in file extension)"
            ))
        })?;
    Ok(extension)
}

pub async fn copy_file(
    destination_directory: SyncPathBuf,
    file_id: String,
    picked_file: PickedFile,
) -> Result<(String, PickedFile), Error> {
    let destination_file_path = AsyncPath::new(&destination_directory)
        .join(&file_id)
        .with_extension(&picked_file.extension);

    println!("destination_file_path: {:?}", destination_file_path);

    if let Some(parent) = destination_file_path.parent() {
        create_dir_all(parent)
            .await
            .map_err(|e| Error::IoError(format!("Failed to create directory: {}", e)))?;
    }
    // TODO: add flag to copy or move the file
    println!("Copying file to destination.");
    async_copy(picked_file.file_path.clone(), &destination_file_path)
        .await
        .map_err(|e| Error::IoError(format!("Failed to copy file: {}", e)))?;

    Ok((file_id, picked_file))
}

/*pub async fn delete_files(file_names: Vec<String>, path: String, id: i32) -> Result<i32, Error> {
    for file_name in file_names {
        let file_path = Path::new(&path).join(file_name);
        remove_file(file_path)
            .await
            .map_err(|e| Error::IoError(format!("Failed to delete file: {}", e)))?;
    }
    Ok(id)
}*/

pub async fn read_zip_file(file_path: &SyncPath) -> Result<Vec<FileInfo>, Error> {
    let file = AsyncFile::open(file_path)
        .await
        .map_err(|_| Error::IoError("Failed opening file.".to_string()))?;
    let mut buffer = Vec::new();
    file.take(10 * 1024 * 1024) // Read up to 10 MB
        .read_to_end(&mut buffer)
        .await
        .map_err(|_| Error::IoError("Failed reaind contents of file.".to_string()))?;

    let reader = Cursor::new(buffer);
    let mut zip = ZipArchive::new(reader).map_err(|_| {
        Error::IoError("Failed creating a Zip archive from the buffer.".to_string())
    })?;

    let mut file_infos = Vec::new();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).map_err(|_| {
            Error::IoError(format!("Failed reading file in index {} in zip file.", i))
        })?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).map_err(|_| {
            Error::IoError(format!(
                "Failed reading file {} from Zip archive.",
                file.name()
            ))
        })?;

        let mut hasher = Sha1::new();
        hasher.update(&contents);
        let checksum = format!("{:x}", hasher.finalize());

        let zipped_file_path = file.enclosed_name().ok_or(Error::IoError(
            "Failed to get the name of the file in the Zip archive.".to_string(),
        ))?;

        let zipped_file_path = zipped_file_path.to_str().ok_or(Error::IoError(
            "Failed to convert file path to string.".to_string(),
        ))?;

        println!("zipped_file_path: {:?}", zipped_file_path);

        file_infos.push(FileInfo {
            name: zipped_file_path.to_string(),
            checksum,
        });
    }

    Ok(file_infos)
}

pub async fn is_zip_file(file_path: &AsyncPath) -> Result<bool, Error> {
    const ZIP_MAGIC_NUMBER: [u8; 4] = [0x50, 0x4B, 0x03, 0x04];

    let mut file = AsyncFile::open(file_path)
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

pub fn is_zip_file_sync(file_path: &SyncPath) -> Result<bool, Error> {
    const ZIP_MAGIC_NUMBER: [u8; 4] = [0x50, 0x4B, 0x03, 0x04];

    let mut file = File::open(file_path)
        .map_err(|_| Error::IoError(format!("Failed opening file {:?}.", file_path.file_name())))?;
    let mut buffer = [0; 4];
    file.read_exact(&mut buffer);

    Ok(buffer == ZIP_MAGIC_NUMBER)
}

/// Extracts the files from the zip files and copies the other files to the destination.
pub fn extract_zip_files(
    files: &Vec<CollectionFile>,
    source: &SyncPathBuf,
    destination: &SyncPathBuf,
) -> Result<(), Error> {
    println!(
        "Extracting zip files from source: {:?}, to destination: {:?}",
        source, destination
    );
    // TODO: no need to extract all files, just the selected one
    // TODO: Or should it be possible for user to select multiple files?
    //       User could add multiple files of the same release and most probably wants to select just one version for running with emulator.
    //       Then again the one version of the same release could consist of multiple files.
    //       But in any case, no need to extract all the files, only the selected ones.
    for file in files {
        let file_path = source.join(&file.get_collection_file_name());
        println!("file_path: {:?}", file_path);
        let res = match is_zip_file_sync(file_path.as_path()) {
            Ok(true) => extract_zip_file(&file_path, &destination),
            Ok(false) => {
                let destination_file = destination.join(&file.original_file_name);
                copy(&file_path, &destination_file)
                    .map_err(|e| Error::IoError(format!("Failed to copy file: {}", e)))?;
                Ok(())
            }
            Err(e) => Err(e),
        };
        if res.is_err() {
            return res;
        }
    }
    println!("Finished extracting zip files");
    Ok(())
}

/// Copies the files to the destination.
pub fn copy_files(
    files: &Vec<CollectionFile>,
    source: &SyncPathBuf,
    destination: &SyncPathBuf,
) -> Result<(), Error> {
    // TODO: no need to copy all files, just the selected one
    for file in files {
        let file_path = source.join(&file.get_collection_file_name());
        println!("file_path: {:?}", file_path);
        let destination_file = destination.join(&file.original_file_name);
        copy(&file_path, &destination_file)
            .map_err(|e| Error::IoError(format!("Failed to copy file: {}", e)))?;
    }
    Ok(())
}

/// Extracts the files from the zip file to the destination.
pub fn extract_zip_file(file_path: &SyncPathBuf, destination: &SyncPathBuf) -> Result<(), Error> {
    println!(
        "Extracting zip file from path: {:?}, to path {:?}",
        file_path, destination
    );
    let file = File::open(&file_path)
        .map_err(|e| Error::IoError(format!("Failed to open file: {}", e)))?;
    let mut buffer = Vec::new();
    file.take(10 * 1024 * 1024) // Read up to 10 MB
        .read_to_end(&mut buffer)
        .map_err(|e| Error::IoError(format!("Failed to read file: {}", e)))?;

    let reader = Cursor::new(buffer);
    let mut zip = ZipArchive::new(reader)
        .map_err(|e| Error::IoError(format!("Failed to create Zip archive: {}", e)))?;

    for i in 0..zip.len() {
        let mut file = zip
            .by_index(i)
            .map_err(|e| Error::IoError(format!("Failed to read file in Zip archive: {}", e)))?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)
            .map_err(|e| Error::IoError(format!("Failed to read file in Zip archive: {}", e)))?;

        let file_path = destination.join(file.name());
        let mut file = File::create(&file_path)
            .map_err(|e| Error::IoError(format!("Failed to create file: {}", e)))?;
        file.write_all(&contents)
            .map_err(|e| Error::IoError(format!("Failed to write file: {}", e)))?;
    }
    Ok(())
}

/*             let file = File::open(&file_path)
                .await
                .map_err(|e| Error::IoError(format!("Failed to open file: {}", e)))?;
            let mut buffer = Vec::new();
            file.take(10 * 1024 * 1024) // Read up to 10 MB
                .read_to_end(&mut buffer)
                .await
                .map_err(|e| Error::IoError(format!("Failed to read file: {}", e)))?;

            let reader = Cursor::new(buffer);
            let mut zip = ZipArchive::new(reader)
                .map_err(|e| Error::IoError(format!("Failed to create Zip archive: {}", e)))?;

            for i in 0..zip.len() {
                let mut file = zip.by_index(i).map_err(|e| {
                    Error::IoError(format!("Failed to read file in Zip archive: {}", e))
                })?;
                let mut contents = Vec::new();
                file.read_to_end(&mut contents).map_err(|e| {
                    Error::IoError(format!("Failed to read file in Zip archive: {}", e))
                })?;

                let mut hasher = Sha1::new();
                hasher.update(&contents);
                let checksum = format!("{:x}", hasher.finalize());

                let file_path = destination.join(file.name());
                let mut file = File::create(&file_path)
                    .await
                    .map_err(|e| Error::IoError(format!("Failed to create file: {}", e)))?;
                file.write_all(&contents)
                    .await
                    .map_err(|e| Error::IoError(format!("Failed to write file: {}", e)))?;
            }
*/
