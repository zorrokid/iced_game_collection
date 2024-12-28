use crate::error::Error;
use crate::model::model::System;
use crate::{database_with_polo::DatabaseWithPolo, model::model::HasOid};
use bson::oid::ObjectId;
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
    EditSystem(ObjectId),
    DeleteSystem(ObjectId),
    Clear,
}

pub enum Action {
    GoHome,
    None,
    EditSystem(ObjectId),
    SystemDeleted,
    SystemSubmitted,
    Error(Error),
}

impl ManageSystems {
    pub fn new(edit_system_id: Option<ObjectId>) -> Result<Self, Error> {
        let db = DatabaseWithPolo::get_instance();
        let systems = db.get_systems()?;
        let edit_system =
            edit_system_id.and_then(|id| systems.iter().find(|system| system.id() == id));
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
                            Err(e) => Action::Error(e),
                        },
                        false => match db.add_system(&self.system) {
                            Ok(_) => Action::SystemSubmitted,
                            Err(e) => Action::Error(e),
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
                    Err(e) => Action::Error(e),
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
                    button("Edit").on_press(Message::EditSystem(system.id())),
                    button("Delete").on_press(Message::DeleteSystem(system.id())),
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
