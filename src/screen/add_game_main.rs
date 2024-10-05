use crate::model::Release;
use crate::screen::add_game_screen::add_game_main_screen;
use crate::screen::add_game_screen::add_release_screen;
use crate::screen::add_game_screen::AddGameScreen;

#[derive(Debug, Clone)]
pub struct AddGameMain {
    screen: AddGameScreen,
    name: String,
    releases: Vec<Release>,
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
}

impl AddGameMain {
    pub fn new() -> Self {
        Self {
            screen: AddGameScreen::AddGameMainScreen(add_game_main_screen::AddGameMainScreen::new(
                std::string::String::new(),
                vec![],
            )),
            name: "".to_string(),
            releases: vec![],
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
                                add_release_screen::AddReleaseScreen::new(),
                            );
                            Action::None
                        }
                        add_game_main_screen::Action::NameChanged(name) => {
                            self.name = name;
                            Action::None
                        }
                        add_game_main_screen::Action::SubmitGame(game) => Action::SubmitGame(game),
                        add_game_main_screen::Action::None => Action::None,
                    }
                } else {
                    Action::None
                }
            }
            Message::AddReleaseScreen(sub_screen2_message) => {
                if let AddGameScreen::AddReleaseScreen(sub_screen2) = &mut self.screen {
                    let action = sub_screen2.update(sub_screen2_message);
                    match action {
                        add_release_screen::Action::ReleaseAdded(name) => {
                            self.releases.push(name);
                            self.screen = AddGameScreen::AddGameMainScreen(
                                add_game_main_screen::AddGameMainScreen::new(
                                    self.name.clone(),
                                    self.releases.clone(),
                                ),
                            );
                            Action::None
                        }
                        add_release_screen::Action::None => Action::None,
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
