use crate::model::{Emulator, Game};
use iced::widget::{button, column, row, text, Column, Row};

pub struct ViewGame {
    game: Game,
    emulators: Vec<Emulator>,
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
    pub fn new(game: Game, emulators: Vec<Emulator>) -> Self {
        Self { game, emulators }
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
            .game
            .releases
            .iter()
            .map(|release| {
                let emulators_for_system = self
                    .emulators
                    .iter()
                    .filter(|emulator| emulator.system_id == release.system.id)
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
        column![title, back_button, Column::with_children(releases_list)].into()
    }
}
