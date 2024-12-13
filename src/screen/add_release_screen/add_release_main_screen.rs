use std::path::PathBuf;
use std::{collections::HashMap, env, vec};

use crate::emulator_runner::EmulatorRunOptions;
use crate::error::Error;
use crate::files::{copy_file, delete_file, pick_file, PickedFile};
use crate::model::{
    collection_file::{CollectionFile, CollectionFileType, GetFileExtensions},
    model::{Emulator, Game, Release, Settings, System},
};
use crate::util::file_path_builder::FilePathBuilder;
use crate::util::image::get_thumbnail_path;
use iced::widget::{button, column, image, pick_list, row, text, text_input, Column};
use iced::{Element, Task};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AddReleaseMainScreen {
    games: Vec<Game>,
    selected_game: Option<Game>,
    release: Release,
    systems: Vec<System>,
    selected_file: HashMap<String, String>,
    emulators: Vec<Emulator>,
    selected_file_type: Option<CollectionFileType>,
    settings: Settings,
    file_path_builder: FilePathBuilder,
}

#[derive(Debug, Clone)]
pub enum Message {
    ManageGames,
    ManageSystems,
    Back,
    GameSelected(Game),
    NameChanged(String),
    SystemSelected(System),
    SelectFile,
    FilePicked(Result<PickedFile, Error>),
    Submit,
    Clear,
    FileSelected(String, String),
    RunWithEmulator(Emulator, String, CollectionFileType),
    CollectionFileTypeSelected(CollectionFileType),
    ViewImage(PathBuf),
    FileCopied(Result<(String, PickedFile), Error>),
    DeleteFile(String),
    FileDeleted(Result<(), Error>, String),
}

pub enum Action {
    ManageGames,
    ManageSystems,
    Back,
    GameSelected(Game),
    NameChanged(String),
    None,
    SystemSelected(System),
    Run(Task<Message>),
    AddFile(CollectionFile),
    Submit(/*Release*/),
    RunWithEmulator(EmulatorRunOptions),
    Clear,
    ViewImage(PathBuf),
    Error(String),
    DeleteFile(CollectionFile),
}

impl AddReleaseMainScreen {
    pub fn new(release: Release) -> Result<Self, Error> {
        let db = crate::database_with_polo::DatabaseWithPolo::get_instance();
        let games = db.get_games()?;
        let systems = db.get_systems()?;
        let emulators = db.get_emulators()?;
        let settings = db.get_settings()?;
        let file_path_builder = FilePathBuilder::new(settings.collection_root_dir.clone());

        Ok(Self {
            games,
            selected_game: None,
            release,
            systems,
            selected_file: HashMap::new(),
            emulators,
            selected_file_type: None,
            settings,
            file_path_builder,
        })
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ManageGames => Action::ManageGames,
            Message::ManageSystems => Action::ManageSystems,
            Message::Back => Action::Back,
            Message::GameSelected(game) => Action::GameSelected(game),
            Message::NameChanged(name) => Action::NameChanged(name),
            Message::SystemSelected(system) => Action::SystemSelected(system),
            Message::SelectFile => Action::Run(Task::perform(pick_file(), Message::FilePicked)),
            Message::FilePicked(result) => {
                let file_id = Uuid::new_v4().to_string();
                let selected_system = self
                    .systems
                    .iter()
                    .find(|system| system.id == self.release.system_id);

                if let (Some(system), Some(selected_file_type)) =
                    (selected_system, self.selected_file_type.clone())
                {
                    match result {
                        Ok(picked_file) => Action::Run(Task::perform(
                            copy_file(
                                self.file_path_builder
                                    .build_target_directory(system, &selected_file_type),
                                file_id,
                                picked_file,
                            ),
                            Message::FileCopied,
                        )),
                        Err(_) => Action::None,
                    }
                } else {
                    Action::None
                }
            }
            Message::FileCopied(result) => match result {
                Ok((id, picked_file)) => {
                    let collection_file = CollectionFile {
                        original_file_name: picked_file.file_name,
                        collection_file_type: self.selected_file_type.clone().unwrap(),
                        files: picked_file.files,
                        is_zip: picked_file.is_zip,
                        id,
                    };
                    Action::AddFile(collection_file)
                }
                Err(_) => Action::None,
            },
            Message::Submit => Action::Submit(/*self.release.clone()*/),
            Message::Clear => Action::Clear,
            Message::FileSelected(id, file) => {
                self.selected_file.insert(id, file);
                Action::None
            }
            Message::RunWithEmulator(emulator, selected_file_name, collection_file_type) => {
                let system = self
                    .systems
                    .iter()
                    .find(|s| s.id == self.release.system_id)
                    .unwrap();
                let options = EmulatorRunOptions {
                    emulator,
                    files: self.release.files.clone(),
                    selected_file_name: selected_file_name,
                    source_path: self
                        .file_path_builder
                        .build_target_directory(system, &collection_file_type),
                    target_path: env::temp_dir(),
                };
                Action::RunWithEmulator(options)
            }
            Message::CollectionFileTypeSelected(file_type) => {
                self.selected_file_type = Some(file_type);
                Action::None
            }
            Message::ViewImage(file_path) => Action::ViewImage(file_path),
            Message::DeleteFile(id) => {
                if let Some(system) = self.systems.iter().find(|s| s.id == self.release.system_id) {
                    if let Some(file) = self.release.files.iter().find(|f| f.id == id) {
                        if let Ok(file_path) = self.file_path_builder.build_file_path(system, file)
                        {
                            return Action::Run(Task::perform(
                                delete_file(file_path.clone()),
                                move |result| Message::FileDeleted(result, id.clone()),
                            ));
                        }
                    }
                }
                Action::None
            }
            Message::FileDeleted(result, id) => match result {
                Ok(_) => {
                    if let Some(file) = self.release.files.iter().find(|f| f.id == id) {
                        return Action::DeleteFile(file.clone());
                    }
                    Action::None
                }
                Err(_) => Action::Error("Failed deleting file".to_string()),
            },
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let back_button = button("Back").on_press(Message::Back);
        let release_name_input_field =
            text_input("Enter release name", &self.release.name).on_input(Message::NameChanged);
        let selected_games_list = self.create_selected_games_list();
        let manage_games_button: button::Button<'_, Message> = button("Manage Games")
            .width(iced::Length::Fixed(200.0))
            .on_press(Message::ManageGames);

        let selected_system = self
            .systems
            .iter()
            .find(|system| system.id == self.release.system_id);

        let systems_select = pick_list(
            self.systems.as_slice(),
            selected_system,
            Message::SystemSelected,
        );
        let manage_systems_button = button("Manage Systems")
            .width(iced::Length::Fixed(200.0))
            .on_press(Message::ManageSystems);

        let file_picker_row = self.create_file_picker();
        let emulator_files_list = self.create_emulator_files_list(&selected_system);
        let scan_files_list = self.create_scan_files_list();

        let main_buttons = row![
            button("Submit").on_press(Message::Submit),
            button("Clear").on_press(Message::Clear)
        ];

        column![
            back_button,
            release_name_input_field,
            selected_games_list,
            manage_games_button,
            systems_select,
            manage_systems_button,
            file_picker_row,
            emulator_files_list,
            scan_files_list,
            main_buttons
        ]
        .into()
    }

    fn create_selected_games_list(&self) -> Element<Message> {
        let selected_games_title = text("Selected Games:");

        let selected_games_list = self
            .release
            .games
            .iter()
            .map(|game_id| {
                let game = self.games.iter().find(|game| game.id == *game_id).unwrap();
                text(&game.name).into()
            })
            .collect::<Vec<Element<Message>>>();

        let available_games: Vec<Game> = self
            .games
            .iter()
            .filter(|g| !self.release.games.contains(&g.id))
            .cloned()
            .collect();

        let game_picker = pick_list(
            available_games,
            self.selected_game.clone(),
            Message::GameSelected,
        );

        column![
            selected_games_title,
            Column::with_children(selected_games_list),
            game_picker
        ]
        .into()
    }

    fn create_file_picker(&self) -> Element<Message> {
        let collection_file_type_picker = pick_list(
            vec![
                CollectionFileType::Rom,
                CollectionFileType::DiskImage,
                CollectionFileType::CoverScan,
                CollectionFileType::Manual,
                CollectionFileType::ScreenShot,
            ],
            self.selected_file_type.clone(),
            Message::CollectionFileTypeSelected,
        );
        let add_file_button = button("Add File").on_press_maybe(
            if !self.release.system_id.is_empty() && self.selected_file_type.is_some() {
                Some(Message::SelectFile)
            } else {
                None
            },
        );
        row![collection_file_type_picker, add_file_button].into()
    }

    fn create_scan_files_list(&self) -> Element<Message> {
        let scan_files_list = self
            .release
            .files
            .iter()
            .filter(|f| f.collection_file_type == CollectionFileType::CoverScan)
            .filter_map(|file| {
                if let Some(system) = self.systems.iter().find(|s| s.id == self.release.system_id) {
                    if let Ok(thumb_path) = get_thumbnail_path(file, &self.settings, system) {
                        if let Ok(file_path) = self.file_path_builder.build_file_path(system, file)
                        {
                            let image = image(thumb_path);
                            let view_image_button =
                                button(image).on_press(Message::ViewImage(file_path));
                            let delete_button =
                                button("Delete").on_press(Message::DeleteFile(file.id.clone()));
                            return Some(row![view_image_button, delete_button].into());
                        }
                    }
                }

                None
            })
            .collect::<Vec<iced::Element<Message>>>();
        Column::with_children(scan_files_list).into()
    }

    fn create_emulator_files_list(&self, selected_system: &Option<&System>) -> Element<Message> {
        let emulators_for_system = if let Some(selected_system) = selected_system {
            self.emulators
                .iter()
                .filter(|emulator| emulator.system_id == selected_system.id)
                .collect::<Vec<&Emulator>>()
        } else {
            vec![]
        };

        // TODO: get file types supported by emulators for the selected system

        let files_list = self
            .release
            .files
            .iter()
            .filter(|f| {
                // TODO: emulator should know it's supported file types and we would filter by the file types supported by emulator
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
                    if self.selected_file.contains_key(file.id.as_str()) {
                        Some(self.selected_file.get(file.id.as_str()).unwrap())
                    } else {
                        None
                    },
                    move |selected_file_name| {
                        Message::FileSelected(file.id.clone(), selected_file_name)
                    },
                );
                let emulator_buttons = emulators_for_system
                    .iter()
                    .filter(|e| {
                        e.supported_file_type_extensions.is_empty()
                            || e.supported_file_type_extensions.contains(
                                &file
                                    .original_file_name
                                    .split('.')
                                    .last()
                                    .unwrap()
                                    .to_string(),
                            )
                            || file.get_file_extensions().into_iter().any(|extension| {
                                e.supported_file_type_extensions.contains(&extension)
                            })
                    })
                    .map(|emulator| {
                        button(emulator.name.as_str())
                            .on_press_maybe({
                                let selected_file = self.selected_file.get(file.id.as_str());
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
                row![
                    container_filename,
                    file_picker,
                    Column::with_children(emulator_buttons)
                ]
                .into()
            })
            .collect::<Vec<iced::Element<Message>>>();
        Column::with_children(files_list).into()
    }
}
