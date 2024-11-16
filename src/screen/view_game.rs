use std::{collections::HashMap, env};

use crate::{
    emulator_runner::EmulatorRunOptions,
    model::{CollectionFile, Emulator, Game, Release, System},
};
use iced::widget::{button, column, pick_list, row, text, Column, Row};

#[derive(Debug, Clone)]
pub struct ViewGame {
    game: Game,
    emulators: Vec<Emulator>,
    releases: Vec<Release>,
    systems: Vec<System>,
    selected_files: HashMap<i32, CollectionFile>,
}

#[derive(Debug, Clone)]
pub enum Message {
    GoToGames,
    RunWithEmulator(EmulatorRunOptions),
    EditRelease(i32),
    FileSelected(i32, CollectionFile),
}

#[derive(Debug, Clone)]
pub enum Action {
    GoToGames,
    RunWithEmulator(EmulatorRunOptions),
    EditRelease(i32),
    None,
}

impl ViewGame {
    pub fn new(game_id: i32) -> Self {
        let db = crate::database::Database::get_instance();
        let game = db.read().unwrap().get_game(game_id).unwrap();
        let emulators = db.read().unwrap().get_emulators();
        let releases = db.read().unwrap().get_releases_with_game(game_id);
        let systems = db.read().unwrap().get_systems();
        Self {
            game,
            emulators,
            releases,
            systems,
            selected_files: HashMap::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::GoToGames => Action::GoToGames,
            Message::RunWithEmulator(options) => Action::RunWithEmulator(options),
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

                let edit_release_button = button("Edit").on_press(Message::EditRelease(release.id));
                let emulator_buttons = emulators_for_system
                    .iter()
                    .map(|emulator| {
                        button(emulator.name.as_str())
                            .on_press_maybe({
                                match self.selected_files.get(&release.id) {
                                    Some(file) => {
                                        Some(Message::RunWithEmulator(EmulatorRunOptions {
                                            emulator: (*emulator).clone(),
                                            files: release.files.clone(),
                                            selected_file_name: file
                                                .file_name
                                                .to_string_lossy()
                                                .to_string(),
                                            source_path: system.roms_destination_path.clone(),
                                            extract_files: (*emulator).extract_files,
                                            target_path: env::temp_dir(),
                                        }))
                                    }
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
                    |file| Message::FileSelected(release.id, file.clone()),
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
