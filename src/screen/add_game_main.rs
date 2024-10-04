use std::ops::Add;

use crate::screen::add_game_screen::sub_screen;
use crate::screen::add_game_screen::sub_screen2;
use crate::screen::add_game_screen::AddGameScreen;

#[derive(Debug, Clone)]
pub struct AddGameMain {
    screen: AddGameScreen,
}

#[derive(Debug, Clone)]
pub enum Message {
    Subscreen(sub_screen::Message),
    Subscreen2(sub_screen2::Message),
}

pub enum Action {
    GoHome,
    None,
}

impl AddGameMain {
    pub fn new() -> Self {
        Self {
            screen: AddGameScreen::SubScreen(sub_screen::SubScreen::new()),
        }
    }

    pub fn title(&self) -> String {
        "Add Game".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Subscreen(sub_screen_message) => {
                if let AddGameScreen::SubScreen(sub_screen) = &mut self.screen {
                    let action = sub_screen.update(sub_screen_message);
                    match action {
                        sub_screen::Action::GoHome => Action::GoHome,
                        sub_screen::Action::GoToSubscreen2 => {
                            self.screen = AddGameScreen::SubScreen2(sub_screen2::SubScreen2::new());
                            Action::None
                        }
                        sub_screen::Action::None => Action::None,
                    }
                } else {
                    Action::None
                }
            }
            Message::Subscreen2(sub_screen2_message) => {
                if let AddGameScreen::SubScreen2(sub_screen2) = &mut self.screen {
                    let action = sub_screen2.update(sub_screen2_message);
                    match action {
                        sub_screen2::Action::GoToSubscreen => {
                            self.screen = AddGameScreen::SubScreen(sub_screen::SubScreen::new());
                            Action::None
                        }
                        sub_screen2::Action::None => Action::None,
                    }
                } else {
                    Action::None
                }
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        match &self.screen {
            AddGameScreen::SubScreen(sub_screen) => sub_screen.view().map(Message::Subscreen),
            AddGameScreen::SubScreen2(sub_screen2) => sub_screen2.view().map(Message::Subscreen2),
        }
    }
}
