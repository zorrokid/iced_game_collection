use crate::screen::games_screen::games_main_screen::GamesMainScreen;
use crate::screen::games_screen::GamesScreen;
use iced::Element;

use super::games_screen::games_main_screen;
use super::view_game;

pub struct GamesMain {
    screen: GamesScreen,
}

#[derive(Debug, Clone)]
pub enum Message {
    GamesMainScreen(games_main_screen::Message),
    ViewGameScreen(view_game::Message),
    None,
}

pub enum Action {
    Back,
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
            _ => Action::None,
        }
    }

    pub fn view(&self) -> Element<Message> {
        match &self.screen {
            GamesScreen::GamesMainScreen(screen) => screen.view().map(Message::GamesMainScreen),
            GamesScreen::ViewGameScreen(screen) => screen.view().map(Message::ViewGameScreen),
        }
    }
}
