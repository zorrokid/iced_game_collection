use crate::{
    error::Error,
    model::{CollectionFile, Emulator},
};
use async_process::Command;
use async_std::path::{Path as AsyncPath, PathBuf as AsyncPathBuf};
use std::path::Path as SyncPath;

#[derive(Debug, Clone)]
pub struct EmulatorRunOptions {
    pub emulator: Emulator,
    pub files: Vec<CollectionFile>, // all files for release?
    pub selected_file: Option<CollectionFile>, // optional single file selected for running
    pub selected_file_name: String, // file name selected for running (either a single file or a file inside a zip archive)
    pub path: String,               // where to find files
    pub extract_files: bool,
}

pub async fn run_with_emulator_async(
    emulator_run_options: EmulatorRunOptions,
) -> Result<(), Error> {
    let EmulatorRunOptions {
        emulator,
        files,
        selected_file,
        selected_file_name,
        path,
        extract_files,
    } = emulator_run_options;
    if files.is_empty() {
        // TODO use other than IoError
        return Err(Error::IoError("No file selected".to_string()));
    }
    let file_path = AsyncPath::new(&path).join(&selected_file_name);
    println!(
        "Running {} with emulator {}",
        file_path.to_string_lossy(),
        emulator.name
    );
    let mut command = Command::new(&emulator.executable)
        .arg(&file_path)
        .arg(&emulator.arguments)
        .spawn()
        .map_err(|e| Error::IoError(format!("Failed to spawn emulator: {}", e)))?;

    let status = command
        .status()
        .await
        .map_err(|e| Error::IoError(format!("Failed to get status of emulator: {}", e)))?;
    println!("Emulator exited with status: {}", status);
    if !status.success() {
        eprintln!("Emulator failed with status: {}", status);
    }
    println!("Finished running with emulator");

    Ok(())
}
