use std::{collections::HashMap, env, sync::Arc};

use iced::{
    widget::{button, pick_list, row, text, Column, Row},
    Element, Task,
};

use crate::{
    database::{
        database_error::DatabaseError, emulator_repository::EmulatorReadRepository,
        repository_manager::RepositoryManager,
    },
    emulator_runner::{process_files_for_emulator, run_with_emulator_async, EmulatorRunOptions},
    error::Error,
    model::{
        collection_file::{CollectionFile, CollectionFileType, GetFileExtension},
        model::Emulator,
    },
};

pub struct EmulatorFilesList {
    system_id: Option<i64>,
    files: Vec<CollectionFile>,
    emulators: Vec<Emulator>,
    selected_file: HashMap<i64, String>,
    repo: Arc<RepositoryManager>,
}

#[derive(Debug, Clone)]
pub enum Message {
    FileSelected(i64, String),
    StartRemoveFile(i64),
    RunWithEmulator(Emulator, String, CollectionFileType),
    FinishedRunningWithEmulator(Result<(), Error>),
    SetFiles(Vec<CollectionFile>, i64),
    EmulatorsLoaded(Result<Vec<Emulator>, DatabaseError>),
}

pub enum Action {
    RemoveFileReference(i64),
    Run(Task<Message>),
    None,
}

impl EmulatorFilesList {
    pub fn new(
        system_id: Option<i64>,
        files: Vec<CollectionFile>,
        repo: Arc<RepositoryManager>,
    ) -> (Self, Task<Message>) {
        let repo_clone = Arc::clone(&repo);
        (
            Self {
                system_id,
                files,
                emulators: vec![],
                selected_file: HashMap::new(),
                repo,
            },
            Task::perform(
                async move { repo_clone.emulators().get_emulators().await },
                Message::EmulatorsLoaded,
            ),
        )
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::FileSelected(id, file) => {
                println!("File selected: {:?}", id);
                self.selected_file.insert(id, file);
                Action::None
            }
            Message::StartRemoveFile(id) => {
                println!("Remove file: {:?}", id);
                Action::RemoveFileReference(id)
            }
            Message::RunWithEmulator(emulator, selected_file_name, selcted_file_type) => {
                if let Some(system_id) = self.system_id {
                    println!(
                        "Run with emulator: {:?} {:?} {:?}",
                        emulator, selected_file_name, selcted_file_type
                    );
                    let filtered_files = self
                        .files
                        .iter()
                        .filter(|f| f.collection_file_type == selcted_file_type)
                        .cloned()
                        .collect::<Vec<CollectionFile>>();

                    let options = EmulatorRunOptions {
                        emulator,
                        files: filtered_files,
                        selected_file_name,
                        source_path: self
                            .file_path_builder
                            .build_target_directory(&system_id, &selcted_file_type),
                        target_path: env::temp_dir(),
                    };

                    if let Err(e) = process_files_for_emulator(&options) {
                        println!("Failed to process files for emulator {:?}", e);
                        return Action::None;
                    }

                    Action::Run(Task::perform(
                        run_with_emulator_async(options),
                        Message::FinishedRunningWithEmulator,
                    ))
                } else {
                    Action::None
                }
            }
            Message::FinishedRunningWithEmulator(result) => {
                if let Err(e) = result {
                    println!("Failed to run with emulator {:?}", e);
                }
                Action::None
            }
            Message::SetFiles(files, system_id) => {
                self.files = files;
                self.system_id = Some(system_id);
                Action::None
            }
            Message::EmulatorsLoaded(result) => {
                self.emulators = match result {
                    Ok(emulators) => emulators,
                    Err(e) => {
                        println!("Failed to load emulators {:?}", e);
                        vec![]
                    }
                };
                Action::None
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let files_list = self
            .files
            .iter()
            .filter(|f| {
                f.collection_file_type == CollectionFileType::Rom
                    || f.collection_file_type == CollectionFileType::DiskImage
                    || f.collection_file_type == CollectionFileType::TapeImage
            })
            .map(|file| {
                let container_filename = text(file.to_string());
                let content_files: Vec<String> = if let Some(files) = &file.files {
                    files.iter().map(|file| file.name.clone()).collect()
                } else {
                    vec![]
                };
                let file_picker = pick_list(
                    content_files,
                    if self.selected_file.contains_key(&file.id()) {
                        Some(self.selected_file.get(&file.id()).unwrap())
                    } else {
                        None
                    },
                    move |selected_file_name| Message::FileSelected(file.id(), selected_file_name),
                );
                let delete_button = button("Remove").on_press(Message::StartRemoveFile(file.id()));
                let emulator_buttons = self.create_emulator_run_buttons(file);
                row![
                    container_filename,
                    file_picker,
                    delete_button,
                    emulator_buttons
                ]
                .into()
            })
            .collect::<Vec<iced::Element<Message>>>();
        Column::with_children(files_list).into()
    }

    fn create_emulator_run_buttons(&self, file: &CollectionFile) -> Element<Message> {
        let mut file_extensions = file.get_file_extensions();
        file_extensions.push(file.get_file_extension());
        let mut file_extensions_iter = file_extensions.iter();

        let buttons = self
            .emulators
            .iter()
            .filter(|e| {
                e.system_id == self.system_id
                    && (e.supported_file_type_extensions.is_empty()
                        || file_extensions_iter
                            .any(|extension| e.supported_file_type_extensions.contains(extension)))
            })
            .map(|emulator| {
                button(emulator.name.as_str())
                    .on_press_maybe({
                        let selected_file = self.selected_file.get(&file.id());
                        match (selected_file, emulator.extract_files) {
                            (Some(file_name), true) => Some(Message::RunWithEmulator(
                                (*emulator).clone(),
                                file_name.clone(),
                                file.collection_file_type.clone(),
                            )),
                            (_, false) => Some(Message::RunWithEmulator(
                                (*emulator).clone(),
                                file.clone().original_file_name,
                                file.collection_file_type.clone(),
                            )),
                            (_, _) => None,
                        }
                    })
                    .into()
            })
            .collect::<Vec<iced::Element<Message>>>();
        Row::with_children(buttons).into()
    }
}
