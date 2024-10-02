use iced::widget::{button, column, text};

pub struct Home {}

#[derive(Debug, Clone)]
pub enum Message {
    AddGame,
    ViewGames,
}

pub enum Action {
    AddGame,
    None,
}

impl Home {
    pub fn new() -> Self {
        Self {}
    }

    pub fn title(&self) -> String {
        "Iced Game Collection".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::AddGame => {
                print!("Add game");
                Action::AddGame
                // Add game
            }
            Message::ViewGames => {
                // View games
                Action::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let add_button = button("Add Game").on_press(Message::AddGame);
        let header = text("Welcome to Iced Game Collection").size(50);
        column![header, add_button].into()
    }
}
