use iced::widget::{button, column, text, text_input};
use iced_game_collection::model::System;

pub struct AddSystem {
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    NameChanged(String),
    Submit,
}

pub enum Action {
    SubmitSystem(System),
    None,
}

impl AddSystem {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
        }
    }

    pub fn title(&self) -> String {
        format!("Add System {}", self.name)
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::NameChanged(name) => {
                self.name = name;
                Action::None
            }
            Message::Submit => Action::SubmitSystem(System {
                id: 0,
                name: self.name.clone(),
            }),
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let header = text("Add system").size(50);
        let name_input_field = text_input("Enter name", &self.name).on_input(Message::NameChanged);
        let add_button = button("Add system").on_press(Message::Submit);
        column![header, name_input_field, add_button].into()
    }
}
