use std::sync::Arc;

use crate::model::model::System;
use crate::service::view_model_service::ViewModelService;
use crate::view_model::list_models::SystemListModel;
use crate::{database::repository_manager::RepositoryManager, error::Error};
use bson::oid::ObjectId;
use iced::widget::{button, column, row, text, text_input, Column};

#[derive(Debug, Clone)]
pub struct ManageSystems {
    system: System,
    systems: Vec<SystemListModel>,
    is_editing: bool,
    repo: Arc<RepositoryManager>,
    view_model_service: Arc<ViewModelService>,
}

#[derive(Debug, Clone)]
pub enum Message {
    NameChanged(String),
    GoHome,
    Submit,
    EditSystem(ObjectId),
    DeleteSystem(ObjectId),
    Clear,
    NotesChanged(String),
}

pub enum Action {
    GoHome,
    None,
    EditSystem(ObjectId),
    SystemSubmitted,
    Error(Error),
}

impl ManageSystems {
    pub fn new(
        repo: Arc<RepositoryManager>,
        view_model_service: Arc<ViewModelService>,
        edit_system_id: Option<ObjectId>,
    ) -> Result<Self, Error> {
        let systems = view_model_service.get_system_list_models()?;
        let edit_system = match edit_system_id {
            Some(id) => repo.systems().get_system(&id)?,
            None => None,
        };

        Ok(Self {
            is_editing: edit_system.is_some(),
            system: match edit_system {
                Some(system) => system.clone(),
                None => System::default(),
            },
            systems,
            repo,
            view_model_service,
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
            Message::NotesChanged(notes) => {
                if !notes.is_empty() {
                    self.system.notes = Some(notes);
                }
                Action::None
            }
            Message::Submit => match &mut self.system.name {
                name if name.is_empty() => Action::None,
                _ => match self.is_editing {
                    true => match self.repo.systems().update_system(&self.system) {
                        Ok(_) => Action::SystemSubmitted,
                        Err(e) => Action::Error(e),
                    },
                    false => match self.repo.systems().add_system(&self.system) {
                        Ok(_) => Action::SystemSubmitted,
                        Err(e) => Action::Error(e),
                    },
                },
            },
            Message::GoHome => Action::GoHome,
            Message::EditSystem(id) => Action::EditSystem(id),
            Message::DeleteSystem(id) => match self.repo.systems().delete_system(&id) {
                Ok(_) => {
                    self.systems.retain(|system| system.id != id);
                    Action::None
                }
                Err(e) => Action::Error(e),
            },
            Message::Clear => {
                self.system = System::default();
                Action::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let name_input_field =
            text_input("Enter name", &self.system.name).on_input(Message::NameChanged);
        let current_notes = self.system.notes.clone().unwrap_or_default();
        let notes_field = text_input("Enter notes", &current_notes).on_input(Message::NotesChanged);
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
                    button("Edit").on_press(Message::EditSystem(system.id)),
                    button("Delete").on_press_maybe(
                        system
                            .can_delete
                            .then_some(Message::DeleteSystem(system.id))
                    ),
                ]
                .into()
            })
            .collect::<Vec<iced::Element<Message>>>();

        let back_button = button("Back").on_press(Message::GoHome);
        column![
            back_button,
            name_input_field,
            notes_field,
            main_buttons,
            Column::with_children(systems_list)
        ]
        .into()
    }
}
