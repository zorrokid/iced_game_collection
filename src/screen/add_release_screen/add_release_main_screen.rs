use std::{env, vec};

use crate::emulator_runner::EmulatorRunOptions;
use crate::error::Error;
use crate::files::pick_file;
use crate::model::{CollectionFile, CollectionFileType, Emulator, Game, Release, System};
use iced::widget::{button, column, pick_list, row, text, text_input, Column};
use iced::{Element, Task};

#[derive(Debug, Clone)]
pub struct AddReleaseMainScreen {
    games: Vec<Game>,
    selected_game: Option<Game>,
    release: Release,
    systems: Vec<System>,
    selected_file: Option<String>,
    emulators: Vec<Emulator>,
    selected_file_type: Option<CollectionFileType>,
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
    FileAdded(Result<CollectionFile, Error>),
    Submit,
    Clear,
    FileSelected(String),
    RunWithEmulator(EmulatorRunOptions),
    CollectionFileTypeSelected(CollectionFileType),
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
    Submit(Release),
    RunWithEmulator(EmulatorRunOptions),
    Clear,
}

impl AddReleaseMainScreen {
    pub fn new(release: Release) -> Self {
        let db = crate::database::Database::get_instance();
        let games = db.read().unwrap().get_games();
        let systems = db.read().unwrap().get_systems();
        let emulators = db.read().unwrap().get_emulators();

        Self {
            games,
            selected_game: None,
            release,
            systems,
            selected_file: None,
            emulators,
            selected_file_type: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ManageGames => Action::ManageGames,
            Message::ManageSystems => Action::ManageSystems,
            Message::Back => Action::Back,
            Message::GameSelected(game) => Action::GameSelected(game),
            Message::NameChanged(name) => Action::NameChanged(name),
            Message::SystemSelected(system) => Action::SystemSelected(system),
            Message::SelectFile => {
                let selected_system = self
                    .systems
                    .iter()
                    .find(|system| system.id == self.release.system_id);
                if let (Some(system), Some(selected_file)) =
                    (selected_system, self.selected_file_type.clone())
                {
                    let source_path = system.roms_source_path.clone();
                    let destination_path = system.roms_destination_path.clone();
                    Action::Run(Task::perform(
                        pick_file(source_path, destination_path, selected_file),
                        Message::FileAdded,
                    ))
                } else {
                    Action::None
                }
            }
            Message::FileAdded(result) => match result {
                Ok(picked_file) => Action::AddFile(picked_file),
                Err(_) => Action::None,
            },
            Message::Submit => Action::Submit(self.release.clone()),
            Message::Clear => Action::Clear,
            Message::FileSelected(file) => {
                self.selected_file = Some(file);
                Action::None
            }
            Message::RunWithEmulator(options) => Action::RunWithEmulator(options),
            Message::CollectionFileTypeSelected(file_type) => {
                self.selected_file_type = Some(file_type);
                Action::None
            }
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
        let files_list = self.create_files_list(&selected_system);

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
            files_list,
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
            vec![CollectionFileType::Rom, CollectionFileType::DiskImage],
            self.selected_file_type.clone(),
            Message::CollectionFileTypeSelected,
        );
        let add_file_button = button("Add File").on_press_maybe(
            if self.release.system_id > 0 && self.selected_file_type.is_some() {
                Some(Message::SelectFile)
            } else {
                None
            },
        );
        row![collection_file_type_picker, add_file_button].into()
    }

    fn create_files_list(&self, selected_system: &Option<&System>) -> Element<Message> {
        let emulators_for_system = if let Some(selected_system) = selected_system {
            self.emulators
                .iter()
                .filter(|emulator| emulator.system_id == selected_system.id)
                .collect::<Vec<&Emulator>>()
        } else {
            vec![]
        };

        let files_list = self
            .release
            .files
            .iter()
            .map(|file| {
                let container_filename = text(file.to_string());
                let content_files: Vec<String> = if let Some(files) = &file.files {
                    files.iter().map(|file| file.name.clone()).collect()
                } else {
                    vec![]
                };
                let file_picker = pick_list(
                    content_files,
                    self.selected_file.clone(),
                    Message::FileSelected,
                );
                let emulator_buttons = emulators_for_system
                    .iter()
                    .map(|emulator| {
                        button(emulator.name.as_str())
                            .on_press_maybe({
                                match (
                                    self.selected_file.clone(),
                                    selected_system,
                                    emulator.extract_files,
                                ) {
                                    (Some(file_name), Some(system), true) => {
                                        Some(Message::RunWithEmulator(EmulatorRunOptions {
                                            emulator: (*emulator).clone(),
                                            files: self.release.files.clone(),
                                            selected_file_name: file_name.clone(),
                                            source_path: system.roms_destination_path.clone(),
                                            target_path: env::temp_dir(),
                                        }))
                                    }
                                    (_, Some(system), false) => {
                                        Some(Message::RunWithEmulator(EmulatorRunOptions {
                                            emulator: (*emulator).clone(),
                                            files: self.release.files.clone(),
                                            selected_file_name: file
                                                .clone()
                                                .file_name
                                                .into_string()
                                                .unwrap(), // TODO: this is not realiable
                                            source_path: system.roms_destination_path.clone(),
                                            target_path: env::temp_dir(),
                                        }))
                                    }
                                    (_, _, _) => None,
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
