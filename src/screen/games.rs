use iced::widget::text;
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
        text("Games").size(50).into()
    }
}
