use crate::model::{Emulator, GameNew, Release};
use iced::widget::{button, column, row, text, Column, Row};

pub struct ViewGame {
    game: GameNew,
    emulators: Vec<Emulator>,
    releases: Vec<Release>,
}

#[derive(Debug, Clone)]
pub enum Message {
    GoToGames,
    RunWithEmulator(Emulator, String),
}

#[derive(Debug, Clone)]
pub enum Action {
    GoToGames,
    RunWithEmulator(Emulator, String),
}

impl ViewGame {
    pub fn new(game: GameNew, emulators: Vec<Emulator>, releases: Vec<Release>) -> Self {
        Self {
            game,
            emulators,
            releases,
        }
    }

    pub fn title(&self) -> String {
        self.game.name.clone()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::GoToGames => Action::GoToGames,
            Message::RunWithEmulator(emulator, file) => Action::RunWithEmulator(emulator, file),
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let title = text(self.game.name.clone()).size(30);
        let releases_list = self
            .releases
            .iter()
            .map(|release| {
                let emulators_for_system = self
                    .emulators
                    .iter()
                    .filter(|emulator| emulator.system_id == release.system_id)
                    .collect::<Vec<&Emulator>>();
                let files_list = release
                    .files
                    .iter()
                    .map(|file| {
                        let emulator_buttons = emulators_for_system
                            .iter()
                            .map(|emulator| {
                                button(emulator.name.as_str())
                                    .on_press(Message::RunWithEmulator(
                                        (*emulator).clone(),
                                        file.clone(),
                                    ))
                                    .into()
                            })
                            .collect::<Vec<iced::Element<Message>>>();

                        row!(text(file), Row::with_children(emulator_buttons),).into()
                    })
                    .collect::<Vec<iced::Element<Message>>>();

                column!(text(release.to_string()), Column::with_children(files_list)).into()
            })
            .collect::<Vec<iced::Element<Message>>>();
        let back_button = button("Back").on_press(Message::GoToGames);
        column![back_button, title, Column::with_children(releases_list)].into()
    }
}
