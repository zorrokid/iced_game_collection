use std::sync::Arc;

use crate::database::repository_manager::RepositoryManager;
use crate::emulator_runner::EmulatorRunOptions;
use crate::error::Error;
use crate::screen::games_screen::games_main_screen::GamesMainScreen;
use crate::screen::games_screen::GamesScreen;
use crate::service::view_model_service::ViewModelService;
use iced::{Element, Task};

use super::games_screen::games_main_screen;
use super::view_game_main;

pub struct GamesMain {
    screen: GamesScreen,
    selected_game_id: Option<i64>,
    repo: Arc<RepositoryManager>,
    view_model_service: Arc<ViewModelService>,
}

#[derive(Debug, Clone)]
pub enum Message {
    GamesMainScreen(games_main_screen::Message),
    ViewGameScreen(view_game_main::Message),
}

pub enum Action {
    Back,
    RunWithEmulator(Box<EmulatorRunOptions>),
    None,
    Run(Task<Message>),
    Error(Error),
}

impl GamesMain {
    pub fn new(
        repo: Arc<RepositoryManager>,
        view_model_service: Arc<ViewModelService>,
    ) -> (Self, Task<Message>) {
        let (screen, task) =
            GamesMainScreen::new(Arc::clone(&repo), Arc::clone(&view_model_service));
        (
            Self {
                screen: GamesScreen::GamesMainScreen(screen),
                selected_game_id: None,
                repo,
                view_model_service,
            },
            task.map(Message::GamesMainScreen),
        )
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
                            self.selected_game_id = Some(id);

                            let (view_game, task) = view_game_main::ViewGameMain::new(
                                id,
                                Arc::clone(&self.repo),
                                Arc::clone(&self.view_model_service),
                            );
                            self.screen = GamesScreen::ViewGameScreen(view_game);
                            Action::Run(task.map(Message::ViewGameScreen))
                        }
                        games_main_screen::Action::GoHome => Action::Back,
                        games_main_screen::Action::Error(error) => Action::Error(error),
                        games_main_screen::Action::None => Action::None,
                        games_main_screen::Action::Run(task) => {
                            Action::Run(task.map(Message::GamesMainScreen))
                        }
                    }
                } else {
                    Action::None
                }
            }
            Message::ViewGameScreen(message) => {
                if let GamesScreen::ViewGameScreen(screen) = &mut self.screen {
                    match screen.update(message) {
                        view_game_main::Action::Back => {
                            let (screen, task) = GamesMainScreen::new(
                                Arc::clone(&self.repo),
                                Arc::clone(&self.view_model_service),
                            );
                            self.screen = GamesScreen::GamesMainScreen(screen);
                            Action::Run(task.map(Message::GamesMainScreen))
                        }
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

    pub fn view(&self) -> Element<Message> {
        match &self.screen {
            GamesScreen::GamesMainScreen(screen) => screen.view().map(Message::GamesMainScreen),
            GamesScreen::ViewGameScreen(screen) => screen.view().map(Message::ViewGameScreen),
        }
    }
}
