use iced::widget::text;
use iced_game_collection::model::Game;
pub struct GameDetails {
    pub game: Game,
}

#[derive(Debug, Clone)]
pub enum Message {
    EditGame(usize),
    DeleteGame(usize),
}

impl GameDetails {
    pub fn new(game: Game) -> Self {
        Self { game }
    }

    pub fn title(&self) -> String {
        self.game.name.clone()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::EditGame(_) => {
                // Edit game in database
            }
            Message::DeleteGame(_) => {
                // Delete game from database
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        text("Game details").into()
    }
}
