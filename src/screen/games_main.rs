use crate::model::{Emulator, PickedFile};
use crate::screen::games_screen::games_main_screen::GamesMainScreen;
use crate::screen::games_screen::GamesScreen;
use iced::Element;

use super::add_release_main;
use super::games_screen::games_main_screen;
use super::view_game;

pub struct GamesMain {
    screen: GamesScreen,
}

#[derive(Debug, Clone)]
pub enum Message {
    GamesMainScreen(games_main_screen::Message),
    ViewGameScreen(view_game::Message),
}

pub enum Action {
    Back,
    RunWithEmulator(Emulator, Vec<PickedFile>, PickedFile, String),
    None,
}

impl GamesMain {
    pub fn new() -> Self {
        Self {
            screen: GamesScreen::GamesMainScreen(GamesMainScreen::new()),
        }
    }

    pub fn title(&self) -> String {
        "Games".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::GamesMainScreen(message) => {
                if let GamesScreen::GamesMainScreen(screen) = &mut self.screen {
                    match screen.update(message) {
                        games_main_screen::Action::ViewGame(id) => {
                            self.screen = GamesScreen::ViewGameScreen(view_game::ViewGame::new(id));
                            Action::None
                        }
                        games_main_screen::Action::GoHome => Action::Back,
                    }
                } else {
                    Action::None
                }
            }
            Message::ViewGameScreen(message) => {
                if let GamesScreen::ViewGameScreen(screen) = &mut self.screen {
                    match screen.update(message) {
                        view_game::Action::GoToGames => {
                            self.screen = GamesScreen::GamesMainScreen(GamesMainScreen::new());
                            Action::None
                        }
                        view_game::Action::RunWithEmulator(
                            emulator,
                            files,
                            selected_file,
                            path,
                        ) => Action::RunWithEmulator(emulator, files, selected_file, path),
                        view_game::Action::EditRelease(id) => Action::None,
                        view_game::Action::None => Action::None,
                    }
                } else {
                    Action::None
                }
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        match &self.screen {
            GamesScreen::GamesMainScreen(screen) => screen.view().map(Message::GamesMainScreen),
            GamesScreen::ViewGameScreen(screen) => screen.view().map(Message::ViewGameScreen),
        }
    }
}
