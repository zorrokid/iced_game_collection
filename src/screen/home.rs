use iced::widget::text;

pub struct Home {}

#[derive(Debug, Clone)]
pub enum Message {
    AddGame,
    ViewGames,
}

impl Home {
    pub fn new() -> Self {
        Self {}
    }

    pub fn title(&self) -> String {
        "Iced Game Collection".to_string()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::AddGame => {
                // Add game
            }
            Message::ViewGames => {
                // View games
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        text("Welcome to Iced Game Collection").size(50).into()
    }
}
