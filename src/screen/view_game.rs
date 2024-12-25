use std::{collections::HashMap, env};

use crate::{
    emulator_runner::EmulatorRunOptions,
    error::Error,
    model::{
        collection_file::CollectionFile,
        model::{Emulator, Game, HasOid, Release, System},
    },
    util::file_path_builder::FilePathBuilder,
};
use bson::oid::ObjectId;
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
    pub fn new(game_id: ObjectId) -> Result<Self, Error> {
        let db = crate::database_with_polo::DatabaseWithPolo::get_instance();
        let emulators = db.get_emulators()?;
        let releases = db.get_releases_with_game(&game_id)?;
        let systems = db.get_systems()?;
        let settings = db.get_settings()?;
        let file_path_builder = FilePathBuilder::new(settings.collection_root_dir.clone());

        let game = db.get_game(&game_id)?;
        match game {
            None => Err(Error::NotFound(format!(
                "Game with id {} not found",
                game_id
            ))),
            Some(game) => Ok(Self {
                game,
                emulators,
                releases,
                systems,
                selected_files: HashMap::new(),
                file_path_builder,
            }),
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::GoToGames => Action::GoToGames,
            Message::RunWithEmulator(emulator, files, file) => {
                let system = self
                    .systems
                    .iter()
                    .find(|system| {
                        emulator
                            .system_id
                            .map_or(false, |system_id| system.id() == system_id)
                    })
                    .unwrap();

                Action::RunWithEmulator(EmulatorRunOptions {
                    emulator,
                    files,
                    selected_file_name: file.original_file_name.clone(),
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
                    .find(|s| {
                        release
                            .system_id
                            .map_or(false, |system_id| s.id() == system_id)
                    })
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
