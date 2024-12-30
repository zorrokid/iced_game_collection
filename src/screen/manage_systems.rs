use crate::error::Error;
use crate::model::model::System;
use crate::view_model::list_models::{get_systems_in_list_model, SystemListModel};
use crate::{database_with_polo::DatabaseWithPolo, model::model::HasOid};
use bson::oid::ObjectId;
use iced::widget::{button, column, row, text, text_input, Column};

#[derive(Debug, Clone)]
pub struct ManageSystems {
    pub system: System,
    pub systems: Vec<SystemListModel>,
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
    SystemSubmitted,
    Error(Error),
}

impl ManageSystems {
    pub fn new(edit_system_id: Option<ObjectId>) -> Result<Self, Error> {
        let db = DatabaseWithPolo::get_instance();
        let systems = get_systems_in_list_model(db)?;
        let edit_system = match edit_system_id {
            Some(id) => db.get_system(&id)?,
            None => None,
        };
        Ok(Self {
            isEditing: edit_system.is_some(),
            system: match edit_system {
                Some(system) => system.clone(),
                None => System::default(),
            },
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
                    Ok(_) => {
                        self.systems.retain(|system| system.id != id);
                        Action::None
                    }
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
                    button("Delete").on_press_maybe(
                        system
                            .can_delete
                            .then(|| Message::DeleteSystem(system.id()))
                    ),
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
