use crate::emulator_runner::EmulatorRunOptions;
use crate::error::Error;
use crate::screen::games_screen::games_main_screen::GamesMainScreen;
use crate::screen::games_screen::GamesScreen;
use iced::{Element, Task};

use super::add_release_main;
use super::games_screen::games_main_screen;
use super::view_game;

pub struct GamesMain {
    screen: GamesScreen,
    selected_game_id: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    GamesMainScreen(games_main_screen::Message),
    ViewGameScreen(view_game::Message),
    EditReleaseScreen(add_release_main::Message),
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
                            match view_game::ViewGame::new(id) {
                                Ok(view_game) => {
                                    self.screen = GamesScreen::ViewGameScreen(view_game);
                                    Action::None
                                }
                                Err(e) => Action::Error(e),
                            }
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
                        view_game::Action::GoToGames => self.create_main_screen(),
                        view_game::Action::RunWithEmulator(options) => {
                            Action::RunWithEmulator(options)
                        }
                        view_game::Action::EditRelease(id) => {
                            match add_release_main::AddReleaseMain::new(Some(id)) {
                                Ok(add_release_main) => {
                                    self.screen = GamesScreen::EditReleaseScreen(add_release_main);
                                    Action::None
                                }
                                Err(e) => Action::Error(e),
                            }
                        }
                        view_game::Action::None => Action::None,
                    }
                } else {
                    Action::None
                }
            }
            Message::EditReleaseScreen(message) => {
                if let GamesScreen::EditReleaseScreen(screen) = &mut self.screen {
                    match screen.update(message) {
                        add_release_main::Action::Back => {
                            if let Some(id) = self.selected_game_id.clone() {
                                match view_game::ViewGame::new(id) {
                                    Ok(view_game) => {
                                        self.screen = GamesScreen::ViewGameScreen(view_game);
                                        Action::None
                                    }
                                    Err(e) => Action::Error(e),
                                }
                            } else {
                                self.create_main_screen()
                            }
                        }
                        add_release_main::Action::ReleaseSubmitted => {
                            if let Some(id) = self.selected_game_id.clone() {
                                match view_game::ViewGame::new(id) {
                                    Ok(view_game) => {
                                        self.screen = GamesScreen::ViewGameScreen(view_game);
                                        Action::None
                                    }
                                    Err(e) => Action::Error(e),
                                }
                            } else {
                                self.create_main_screen()
                            }
                        }
                        add_release_main::Action::Run(task) => {
                            Action::Run(task.map(Message::EditReleaseScreen))
                        }
                        add_release_main::Action::RunWithEmulator(options) => {
                            Action::RunWithEmulator(options)
                        }

                        add_release_main::Action::None => Action::None,
                        add_release_main::Action::Error(error) => Action::None,
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
            GamesScreen::EditReleaseScreen(screen) => screen.view().map(Message::EditReleaseScreen),
        }
    }
}
