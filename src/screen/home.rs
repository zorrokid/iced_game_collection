use iced::widget::{button, column, text, text_input};

pub struct Home {
    name: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddGame,
    ViewGames,
    NameChanged(String),
}

pub enum Action {
    AddGame(String),
    None,
}

impl Home {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
        }
    }

    pub fn title(&self) -> String {
        "Iced Game Collection".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::AddGame => {
                print!("Add game");
                Action::AddGame(self.name.clone())
                // Add game
            }
            Message::ViewGames => {
                // View games
                Action::None
            }
            Message::NameChanged(name) => {
                self.name = name;
                Action::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let name_input_field = text_input("Enter name", &self.name).on_input(Message::NameChanged);
        let add_button = button("Add Game").on_press(Message::AddGame);
        let header = text("Welcome to Iced Game Collection").size(50);
        column![header, name_input_field, add_button].into()
    }
}
