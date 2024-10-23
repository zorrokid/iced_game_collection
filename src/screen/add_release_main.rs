use crate::screen::add_release_screen::add_release_main_screen;
use crate::screen::add_release_screen::manage_games_screen;
use crate::screen::add_release_screen::manage_systems_screen;
use crate::screen::add_release_screen::AddReleaseScreen;
use iced::{Element, Task};

#[derive(Debug, Clone)]
pub struct AddReleaseMain {
    screen: AddReleaseScreen,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddReleaseMainScreen(add_release_main_screen::Message),
    ManageGamesScreen(manage_games_screen::Message),
    ManageSystemsScreen(manage_systems_screen::Message),
}

pub enum Action {
    Back,
    SubmitRelease(crate::model::Release),
    None,
    Run(Task<Message>),
    Error(String),
}

impl AddReleaseMain {
    pub fn new() -> Self {
        Self {
            screen: AddReleaseScreen::AddReleaseMainScreen(
                add_release_main_screen::AddReleaseMainScreen::new(),
            ),
        }
    }

    pub fn title(&self) -> String {
        "Add Release".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::AddReleaseMainScreen(sub_screen_message) => {
                if let AddReleaseScreen::AddReleaseMainScreen(sub_screen) = &mut self.screen {
                    let action = sub_screen.update(sub_screen_message);
                    match action {
                        add_release_main_screen::Action::ManageGames => {
                            self.screen = AddReleaseScreen::ManageGamesScreen(
                                manage_games_screen::ManageGamesScreen::new(),
                            );
                        }
                        add_release_main_screen::Action::ManageSystems => {
                            self.screen = AddReleaseScreen::ManageSystemsScreen(
                                manage_systems_screen::ManageSystemsScreen::new(),
                            );
                        }
                        add_release_main_screen::Action::Back => {
                            return Action::Back;
                        }
                    }
                }
            }
            Message::ManageGamesScreen(sub_screen_message) => {
                if let AddReleaseScreen::ManageGamesScreen(sub_screen) = &mut self.screen {
                    let action = sub_screen.update(sub_screen_message);
                    match action {
                        manage_games_screen::Action::Back => {
                            self.screen = AddReleaseScreen::AddReleaseMainScreen(
                                add_release_main_screen::AddReleaseMainScreen::new(),
                            );
                        }
                    }
                }
            }
            Message::ManageSystemsScreen(sub_screen_message) => {
                if let AddReleaseScreen::ManageSystemsScreen(sub_screen) = &mut self.screen {
                    let action = sub_screen.update(sub_screen_message);
                    match action {
                        manage_systems_screen::Action::Back => {
                            self.screen = AddReleaseScreen::AddReleaseMainScreen(
                                add_release_main_screen::AddReleaseMainScreen::new(),
                            );
                        }
                    }
                }
            }
        }
        Action::None
    }

    pub fn view(&self) -> Element<Message> {
        match &self.screen {
            AddReleaseScreen::AddReleaseMainScreen(screen) => {
                screen.view().map(Message::AddReleaseMainScreen)
            }
            AddReleaseScreen::ManageGamesScreen(screen) => {
                screen.view().map(Message::ManageGamesScreen)
            }
            AddReleaseScreen::ManageSystemsScreen(screen) => {
                screen.view().map(Message::ManageSystemsScreen)
            }
        }
    }
}
