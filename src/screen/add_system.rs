use crate::model::System;
use iced::widget::{button, column, text, text_input, Column};

// TODO: move AddSystem under Add Release
pub struct AddSystem {
    pub name: String,
    pub systems: Vec<System>,
    pub error: Option<String>,
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
    pub fn new(systems: Vec<System>) -> Self {
        Self {
            name: "".to_string(),
            systems,
            error: None,
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
            Message::Submit => {
                if self.name.is_empty() {
                    self.error = Some("Name cannot be empty".to_string());
                    return Action::None;
                } else {
                    Action::SubmitSystem(System {
                        id: 0,
                        name: self.name.clone(),
                    })
                }
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let header = text("Add system").size(50);
        let name_input_field = text_input("Enter name", &self.name).on_input(Message::NameChanged);
        let add_button = button("Add system").on_press(Message::Submit);
        let systems_list = self
            .systems
            .iter()
            .map(|system| text(system.to_string()).into())
            .collect::<Vec<iced::Element<Message>>>();
        let error = if let Some(error) = &self.error {
            text(error)
        } else {
            text("")
        };
        column![
            header,
            error,
            name_input_field,
            add_button,
            Column::with_children(systems_list)
        ]
        .into()
    }
}
