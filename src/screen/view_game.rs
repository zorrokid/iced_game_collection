use std::{collections::HashMap, env, ptr::read};

use crate::{
    emulator_runner::EmulatorRunOptions,
    model::{CollectionFile, Emulator, Game, Release, System},
    util::file_path_builder::FilePathBuilder,
};
use iced::widget::{button, column, pick_list, row, text, Column, Row};

#[derive(Debug, Clone)]
pub struct ViewGame {
    game: Game,
    emulators: Vec<Emulator>,
    releases: Vec<Release>,
    systems: Vec<System>,
    selected_files: HashMap<String, CollectionFile>,
    file_path_builder: FilePathBuilder,
}

#[derive(Debug, Clone)]
pub enum Message {
    GoToGames,
    RunWithEmulator(Emulator, Vec<CollectionFile>, CollectionFile),
    EditRelease(String),
    FileSelected(String, CollectionFile),
}

#[derive(Debug, Clone)]
pub enum Action {
    GoToGames,
    RunWithEmulator(EmulatorRunOptions),
    EditRelease(String),
    None,
}

impl ViewGame {
    pub fn new(game_id: String) -> Self {
        let db = crate::database::Database::get_instance();
        let read_handle = db.read().unwrap();

        let game = read_handle.get_game(&game_id).unwrap();
        let emulators = read_handle.get_emulators();
        let releases = read_handle.get_releases_with_game(&game_id);
        let systems = read_handle.get_systems();
        let settings = read_handle.get_settings();
        let file_path_builder = FilePathBuilder::new(settings.collection_root_dir.clone());

        Self {
            game,
            emulators,
            releases,
            systems,
            selected_files: HashMap::new(),
            file_path_builder,
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::GoToGames => Action::GoToGames,
            Message::RunWithEmulator(emulator, files, file) => {
                let system = self
                    .systems
                    .iter()
                    .find(|system| system.id == emulator.system_id)
                    .unwrap();

                Action::RunWithEmulator(EmulatorRunOptions {
                    emulator,
                    files,
                    selected_file_name: file.file_name.clone(),
                    source_path: self
                        .file_path_builder
                        .build_target_directory(system, &file.collection_file_type),
                    target_path: env::temp_dir(),
                })
            }
            Message::EditRelease(id) => Action::EditRelease(id),
            Message::FileSelected(id, file_name) => {
                self.selected_files.insert(id, file_name);
                Action::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let title = text(self.game.name.clone()).size(30);

        let releases_list = self
            .releases
            .iter()
            .map(|release| {
                let system = self
                    .systems
                    .iter()
                    .find(|s| s.id == release.system_id)
                    .unwrap();

                let emulators_for_system = self
                    .emulators
                    .iter()
                    .filter(|emulator| emulator.system_id == release.system_id)
                    .collect::<Vec<&Emulator>>();

                let edit_release_button =
                    button("Edit").on_press(Message::EditRelease(release.id.clone()));
                let emulator_buttons = emulators_for_system
                    .iter()
                    .map(|emulator| {
                        button(emulator.name.as_str())
                            .on_press_maybe({
                                match self.selected_files.get(&release.id) {
                                    Some(file) => Some(Message::RunWithEmulator(
                                        (*emulator).clone(),
                                        release.files.clone(),
                                        file.clone(),
                                    )),
                                    None => None,
                                }
                            })
                            .into()
                    })
                    .collect::<Vec<iced::Element<Message>>>();

                let files_pick_list = pick_list(
                    release.files.as_slice(),
                    if self.selected_files.contains_key(&release.id) {
                        Some(self.selected_files.get(&release.id).unwrap())
                    } else {
                        None
                    },
                    |file| Message::FileSelected(release.id.clone(), file.clone()),
                );

                let release_row = row![
                    text(release.to_string()),
                    text(system.name.clone()),
                    edit_release_button,
                    files_pick_list,
                    Row::with_children(emulator_buttons)
                ];

                release_row.into()
            })
            .collect::<Vec<iced::Element<Message>>>();
        let back_button = button("Back").on_press(Message::GoToGames);
        column![back_button, title, Column::with_children(releases_list)].into()
    }
}
