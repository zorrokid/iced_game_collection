use crate::{
    error::Error,
    model::{Emulator, PickedFile},
};
use async_process::Command;
use async_std::path::Path;

#[derive(Debug, Clone)]
pub struct EmulatorRunOptions {
    pub emulator: Emulator,
    pub files: Vec<PickedFile>,
    pub selected_file: PickedFile,
    pub selected_file_name: Option<String>,
    pub path: String,
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
    } = emulator_run_options;
    if files.is_empty() {
        // TODO use other than IoError
        return Err(Error::IoError("No file selected".to_string()));
    }
    let file_path = Path::new(&path).join(&selected_file.file_name);
    println!("Running {} with emulator {}", selected_file, emulator.name);
    let mut child = Command::new(&emulator.executable)
        .arg(&file_path)
        .arg(&emulator.arguments)
        .spawn()
        .map_err(|e| Error::IoError(format!("Failed to spawn emulator: {}", e)))?;

    let status = child
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
