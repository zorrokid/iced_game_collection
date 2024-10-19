use std::vec;

use crate::model::{get_new_id, Game, System};
use crate::screen::add_game_screen::add_game_main_screen;
use crate::screen::add_game_screen::add_release_screen;
use crate::screen::add_game_screen::AddGameScreen;
use iced::Task;

#[derive(Debug, Clone)]
pub struct AddGameMain {
    screen: AddGameScreen,
    systems: Vec<System>,
    game: Game,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddGameMainScreen(add_game_main_screen::Message),
    AddReleaseScreen(add_release_screen::Message),
}

pub enum Action {
    GoHome,
    SubmitGame(crate::model::Game),
    None,
    Run(Task<Message>),
}

impl AddGameMain {
    pub fn new(systems: Vec<System>, games: Vec<Game>, edit_game: Option<Game>) -> Self {
        let game = match edit_game {
            Some(game) => game,
            None => Game {
                id: get_new_id(&games),
                name: "".to_string(),
                releases: vec![],
            },
        };
        Self {
            screen: AddGameScreen::AddGameMainScreen(add_game_main_screen::AddGameMainScreen::new(
                game.clone(),
            )),
            game,
            systems,
        }
    }

    pub fn title(&self) -> String {
        "Add Game".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::AddGameMainScreen(sub_screen_message) => {
                if let AddGameScreen::AddGameMainScreen(sub_screen) = &mut self.screen {
                    let action = sub_screen.update(sub_screen_message);
                    match action {
                        add_game_main_screen::Action::GoHome => Action::GoHome,
                        add_game_main_screen::Action::AddRelease => {
                            self.screen = AddGameScreen::AddReleaseScreen(
                                add_release_screen::AddReleaseScreen::new(self.systems.clone()),
                            );
                            Action::None
                        }
                        add_game_main_screen::Action::NameChanged(name) => {
                            self.game.name = name;
                            Action::None
                        }
                        add_game_main_screen::Action::SubmitGame(game) => Action::SubmitGame(game),
                    }
                } else {
                    Action::None
                }
            }
            Message::AddReleaseScreen(add_release_screen_message) => {
                if let AddGameScreen::AddReleaseScreen(add_release_screen) = &mut self.screen {
                    let action = add_release_screen.update(add_release_screen_message);
                    match action {
                        add_release_screen::Action::ReleaseAdded(name) => {
                            self.game.releases.push(name);
                            self.screen = AddGameScreen::AddGameMainScreen(
                                add_game_main_screen::AddGameMainScreen::new(self.game.clone()),
                            );
                            Action::None
                        }
                        add_release_screen::Action::None => Action::None,
                        add_release_screen::Action::GoBack => {
                            self.screen = AddGameScreen::AddGameMainScreen(
                                add_game_main_screen::AddGameMainScreen::new(self.game.clone()),
                            );
                            Action::None
                        }
                        add_release_screen::Action::Run(task) => {
                            Action::Run(task.map(Message::AddReleaseScreen))
                        }
                    }
                } else {
                    Action::None
                }
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        match &self.screen {
            AddGameScreen::AddGameMainScreen(sub_screen) => {
                sub_screen.view().map(Message::AddGameMainScreen)
            }
            AddGameScreen::AddReleaseScreen(sub_screen2) => {
                sub_screen2.view().map(Message::AddReleaseScreen)
            }
        }
    }
}
