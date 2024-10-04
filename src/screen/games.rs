use crate::model::Game;
use iced::widget::{button, column, text, Column};
use iced::Element;
pub struct Games {
    pub games: Vec<Game>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ViewGame(usize),
    EditGame(usize),
    DeleteGame(usize),
    GoHome,
}

pub enum Action {
    GoHome,
    None,
}

impl Games {
    pub fn new(games: Vec<Game>) -> Self {
        Self { games }
    }

    pub fn title(&self) -> String {
        "Games".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ViewGame(index) => {
                // View game
                Action::None
            }
            Message::EditGame(index) => {
                // Edit game
                Action::None
            }
            Message::DeleteGame(index) => {
                // Delete game
                Action::None
            }
            Message::GoHome => Action::GoHome,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        // list of games
        let games = self.games.iter().map(|game| text(game.name.clone()).into());
        let games_list_with_container =
            Column::with_children(games.collect::<Vec<Element<Message>>>());
        let home_button = button("Home").on_press(Message::GoHome);
        column![games_list_with_container, home_button].into()
    }
}
