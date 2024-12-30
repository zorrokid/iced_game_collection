use crate::emulator_runner::EmulatorRunOptions;
use crate::error::Error;
use crate::screen::games_screen::games_main_screen::GamesMainScreen;
use crate::screen::games_screen::GamesScreen;
use bson::oid::ObjectId;
use iced::{Element, Task};

use super::games_screen::games_main_screen;
use super::view_game_main;

pub struct GamesMain {
    screen: GamesScreen,
    selected_game_id: Option<ObjectId>,
}

#[derive(Debug, Clone)]
pub enum Message {
    GamesMainScreen(games_main_screen::Message),
    ViewGameScreen(view_game_main::Message),
}

pub enum Action {
    Back,
    RunWithEmulator(EmulatorRunOptions),
    None,
    Run(Task<Message>),
    Error(Error),
}

impl GamesMain {
    pub fn new() -> Result<Self, Error> {
        let screen = GamesMainScreen::new()?;
        Ok(Self {
            screen: GamesScreen::GamesMainScreen(screen),
            selected_game_id: None,
        })
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
                            self.selected_game_id = Some(id.clone());
                            match view_game_main::ViewGameMain::new(id) {
                                Ok(view_game) => {
                                    self.screen = GamesScreen::ViewGameScreen(view_game);
                                    Action::None
                                }
                                Err(e) => Action::Error(e),
                            }
                        }
                        games_main_screen::Action::GoHome => Action::Back,
                        games_main_screen::Action::Error(error) => Action::Error(error),
                        games_main_screen::Action::None => Action::None,
                    }
                } else {
                    Action::None
                }
            }
            Message::ViewGameScreen(message) => {
                if let GamesScreen::ViewGameScreen(screen) = &mut self.screen {
                    match screen.update(message) {
                        view_game_main::Action::Back => self.create_main_screen(),
                        view_game_main::Action::Run(task) => {
                            Action::Run(task.map(Message::ViewGameScreen))
                        }
                        view_game_main::Action::None => Action::None,
                        view_game_main::Action::Error(error) => Action::Error(error),
                        view_game_main::Action::RunWithEmulator(options) => {
                            Action::RunWithEmulator(options)
                        }
                    }
                } else {
                    Action::None
                }
            }
        }
    }

    fn create_main_screen(&mut self) -> Action {
        match GamesMainScreen::new() {
            Ok(screen) => {
                self.screen = GamesScreen::GamesMainScreen(screen);
                Action::None
            }
            Err(e) => Action::Error(e),
        }
    }

    pub fn view(&self) -> Element<Message> {
        match &self.screen {
            GamesScreen::GamesMainScreen(screen) => screen.view().map(Message::GamesMainScreen),
            GamesScreen::ViewGameScreen(screen) => screen.view().map(Message::ViewGameScreen),
        }
    }
}
