use crate::model::{Game, Release};
use iced::widget::{button, column, text, text_input};

#[derive(Debug, Clone)]
pub struct AddGame {
    pub name: String,
    pub releases: Vec<Release>,
}

#[derive(Debug, Clone)]
pub enum Message {
    NameChanged(String),
    Submit,
    AddRelease,
}

pub enum Action {
    SubmitGame(Game),
    None,
    AddRelease(AddGame),
}

impl AddGame {
    pub fn new(state: Option<AddGame>) -> Self {
        if let Some(state) = state {
            state
        } else {
            Self {
                name: "".to_string(),
                releases: vec![],
            }
        }
    }

    pub fn title(&self) -> String {
        format!("Add Game {}", self.name)
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::NameChanged(name) => {
                self.name = name;
                Action::None
            }
            Message::Submit => Action::SubmitGame(Game {
                id: 0,
                name: self.name.clone(),
            }),
            Message::AddRelease => Action::AddRelease(self.clone()),
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let header = text("Add game").size(50);
        let name_input_field = text_input("Enter name", &self.name).on_input(Message::NameChanged);
        let add_release_button = button("Add Release").on_press(Message::AddRelease);
        let add_button = button("Add Game").on_press(Message::Submit);
        column![header, name_input_field, add_release_button, add_button].into()
    }
}
