use std::{collections::HashMap, env};

use bson::oid::ObjectId;
use iced::{
    widget::{button, pick_list, row, text, Column, Row},
    Element, Task,
};

use crate::{
    emulator_runner::{process_files_for_emulator, run_with_emulator_async, EmulatorRunOptions},
    error::Error,
    model::{
        collection_file::{
            CollectionFile, CollectionFileType, GetFileExtension, GetFileExtensions as _,
        },
        model::{Emulator, HasOid as _, Settings},
    },
    util::file_path_builder::FilePathBuilder,
};

pub struct EmulatorFilesList {
    system_id: Option<ObjectId>,
    files: Vec<CollectionFile>,
    emulators: Vec<Emulator>,
    selected_file: HashMap<ObjectId, String>,
    file_path_builder: FilePathBuilder,
}

#[derive(Debug, Clone)]
pub enum Message {
    FileSelected(ObjectId, String),
    StartRemoveFile(ObjectId),
    RunWithEmulator(Emulator, String, CollectionFileType),
    FinishedRunningWithEmulator(Result<(), Error>),
    SetFiles(Vec<CollectionFile>, ObjectId),
}

pub enum Action {
    RemoveFileReference(ObjectId),
    Run(Task<Message>),
    None,
}

impl EmulatorFilesList {
    pub fn new(system_id: Option<ObjectId>, files: Vec<CollectionFile>) -> Self {
        let db = crate::database_with_polo::DatabaseWithPolo::get_instance();
        let emulators = db.get_emulators().unwrap_or_else(|err| {
            println!("Failed to get emulators {:?}", err);
            vec![]
        });

        let settings = db.get_settings().unwrap_or_else(|err| {
            println!("Failed to get settings {:?}", err);
            Settings::default()
        });

        let file_path_builder = FilePathBuilder::new(settings.collection_root_dir.clone());

        Self {
            system_id,
            files,
            emulators,
            selected_file: HashMap::new(),
            file_path_builder,
        }
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
