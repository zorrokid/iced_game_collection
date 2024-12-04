use crate::database::Database;
use crate::model::model::System;
use iced::widget::{button, column, row, text, text_input, Column};

#[derive(Debug, Clone)]
pub struct ManageSystems {
    pub system: System,
    pub systems: Vec<System>,
}

#[derive(Debug, Clone)]
pub enum Message {
    NameChanged(String),
    GoHome,
    Submit,
    EditSystem(String),
    DeleteSystem(String),
    Clear,
}

pub enum Action {
    GoHome,
    None,
    EditSystem(String),
    SystemDeleted,
    SystemSubmitted,
}

impl ManageSystems {
    pub fn new(edit_system_id: Option<String>) -> Self {
        let db = Database::get_instance();
        let systems = db.read().unwrap().get_systems();
        let edit_system = edit_system_id.and_then(|id| db.read().unwrap().get_system(&id));
        Self {
            system: match edit_system {
                Some(system) => system,
                None => System::default(),
            },
            systems,
        }
    }

    pub fn title(&self) -> String {
        "Manage systems".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::NameChanged(name) => {
                self.system.name = name;
                Action::None
            }
            Message::Submit => match &mut self.system.name {
                name if name.is_empty() => Action::None,
                _ => {
                    let db = Database::get_instance();
                    db.write()
                        .unwrap()
                        .add_or_update_system(self.system.clone());
                    Action::SystemSubmitted
                }
            },
            Message::GoHome => Action::GoHome,
            Message::EditSystem(id) => Action::EditSystem(id),
            Message::DeleteSystem(id) => {
                let db = Database::get_instance();
                db.write().unwrap().delete_system(&id);
                Action::SystemDeleted
            }
            Message::Clear => {
                self.system = System::default();
                Action::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let name_input_field =
            text_input("Enter name", &self.system.name).on_input(Message::NameChanged);
        let main_buttons = row![
            button("Submit").on_press(Message::Submit),
            button("Clear").on_press(Message::Clear)
        ];
        let systems_list = self
            .systems
            .iter()
            .map(|system| {
                row![
                    text(system.to_string()).width(iced::Length::Fixed(300.0)),
                    button("Edit").on_press(Message::EditSystem(system.id.clone())),
                    button("Delete").on_press(Message::DeleteSystem(system.id.clone())),
                ]
                .into()
            })
            .collect::<Vec<iced::Element<Message>>>();

        let back_button = button("Back").on_press(Message::GoHome);
        column![
            back_button,
            name_input_field,
            main_buttons,
            Column::with_children(systems_list)
        ]
        .into()
    }
}
