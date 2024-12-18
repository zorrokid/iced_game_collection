use crate::database_with_polo::DatabaseWithPolo;
use crate::error::Error;
use crate::model::model::System;
use iced::widget::{button, column, row, text, text_input, Column};

#[derive(Debug, Clone)]
pub struct ManageSystems {
    pub system: System,
    pub systems: Vec<System>,
    pub isEditing: bool,
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
    Error(String),
}

impl ManageSystems {
    pub fn new(edit_system_id: Option<String>) -> Result<Self, Error> {
        let db = DatabaseWithPolo::get_instance();
        let systems = db.get_systems()?;
        let edit_system =
            edit_system_id.and_then(|id| systems.iter().find(|system| system.id == id));
        Ok(Self {
            system: match edit_system {
                Some(system) => system.clone(),
                None => System::default(),
            },
            isEditing: edit_system.is_some(),
            systems,
        })
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
                    let db = DatabaseWithPolo::get_instance();
                    match self.isEditing {
                        true => match db.update_system(&self.system) {
                            Ok(_) => Action::SystemSubmitted,
                            Err(e) => Action::Error(e.to_string()),
                        },
                        false => match db.add_system(&self.system) {
                            Ok(_) => Action::SystemSubmitted,
                            Err(e) => Action::Error(e.to_string()),
                        },
                    }
                }
            },
            Message::GoHome => Action::GoHome,
            Message::EditSystem(id) => Action::EditSystem(id),
            Message::DeleteSystem(id) => {
                let db = DatabaseWithPolo::get_instance();
                match db.delete_system(&id) {
                    Ok(_) => Action::SystemDeleted,
                    Err(e) => Action::Error(e.to_string()),
                }
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
