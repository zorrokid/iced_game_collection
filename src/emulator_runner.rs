use crate::{
    error::Error,
    files::{copy_files, extract_zip_files},
    model::{CollectionFile, Emulator},
};
use async_process::Command;
use async_std::path::Path as AsyncPath;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct EmulatorRunOptions {
    pub emulator: Emulator,
    pub files: Vec<CollectionFile>, // all files for release?
    pub selected_file_name: String, // file name selected for running (either a single file or a file inside a zip archive)
    pub source_path: PathBuf,       // where to find files
    pub target_path: PathBuf,       // where to extract / copy files
}

pub async fn run_with_emulator_async(
    emulator_run_options: EmulatorRunOptions,
) -> Result<(), Error> {
    let EmulatorRunOptions {
        emulator,
        files,
        selected_file_name,
        source_path,
        target_path,
    } = emulator_run_options;
    if files.is_empty() {
        // TODO use other than IoError
        return Err(Error::IoError("No file selected".to_string()));
    }
    let file_path = AsyncPath::new(&target_path).join(&selected_file_name);
    println!(
        "Running {} with emulator {}",
        file_path.to_string_lossy(),
        emulator.executable
    );

    let mut command = Command::new(&emulator.executable);

    command.arg(&file_path).current_dir(target_path);

    if emulator.arguments.len() > 0 {
        // TODO: should use command.args() instead and emulator arguments should be split into separate strings
        command.arg(&emulator.arguments);
    }

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

pub fn process_files_for_emulator(options: &EmulatorRunOptions) -> Result<(), Error> {
    let source_path = PathBuf::from(&options.source_path); // .join(&options.selected_file.file_name);
    if options.emulator.extract_files {
        // TODO: extract all files or only selected_file?
        extract_zip_files(&options.files, &source_path, &options.target_path)
    } else {
        copy_files(&options.files, &source_path, &options.target_path)
    }
}
