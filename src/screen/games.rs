use iced::widget::{column, text, Column};
use iced::Element;
use iced_game_collection::model::Game;
pub struct Games {
    pub games: Vec<Game>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ViewGame(usize),
    EditGame(usize),
    DeleteGame(usize),
}

impl Games {
    pub fn new(games: Vec<Game>) -> Self {
        Self { games }
    }

    pub fn title(&self) -> String {
        "Games".to_string()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ViewGame(index) => {
                // View game
            }
            Message::EditGame(index) => {
                // Edit game
            }
            Message::DeleteGame(index) => {
                // Delete game
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        // list of games
        let games = self.games.iter().map(|game| text(game.name.clone()).into());
        let games_list_with_container =
            Column::with_children(games.collect::<Vec<Element<Message>>>());
        column![games_list_with_container].into()
    }
}
