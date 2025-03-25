use std::sync::Arc;

use crate::database::database_error::DatabaseError;
use crate::database::repository_manager::RepositoryManager;
use crate::database::software_title_repository::SoftwareTitleWriteRepository as _;
use crate::error::Error;
use crate::model::model::SoftwareTitle;
use crate::service::view_model_service::ViewModelService;
use crate::view_model::list_models::SoftwareTitleListModel;
use iced::widget::{button, column, row, text, text_input, Column};
use iced::{Element, Task};

#[derive(Debug, Clone)]
pub struct ManageGames {
    software_titles: Vec<SoftwareTitleListModel>,
    software_title: SoftwareTitle,
    is_edit: bool,
    repo: Arc<RepositoryManager>,
    view_model_service: Arc<ViewModelService>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Back,
    SubmitSoftwareTitle,
    DeleteSoftwareTitle(i64),
    EditSoftwareTitle(i64),
    NameChanged(String),
    Clear,
    SoftwareTitleListModelsLoaded(Result<Vec<SoftwareTitleListModel>, DatabaseError>),
}

#[derive(Debug, Clone)]
pub enum Action {
    Back,
    None,
    GameSubmitted,
    GameDeleted,
    Error(Error),
}

impl ManageGames {
    pub fn new(
        repo: Arc<RepositoryManager>,
        view_model_service: Arc<ViewModelService>,
        edit_software_title: Option<SoftwareTitle>,
    ) -> (Self, Task<Message>) {
        let is_edit = edit_software_title.is_some();
        let view_model_service_clone = Arc::clone(&view_model_service);
        let software_title_list_models_task = Task::perform(
            async move {
                view_model_service_clone
                    .get_software_title_list_models()
                    .await
            },
            Message::SoftwareTitleListModelsLoaded,
        );
        (
            Self {
                software_title: edit_software_title.unwrap_or_default(),
                software_titles: vec![],
                is_edit,
                repo,
                view_model_service,
            },
            software_title_list_models_task,
        )
    }

    pub fn title(&self) -> String {
        "Manage Games".to_string()
    }

    fn update_games(&mut self) -> Result<(), Error> {
        let software_titles = self.view_model_service.get_software_title_list_models()?;
        self.software_titles = software_titles;
        Ok(())
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Back => Action::Back,
            Message::SubmitSoftwareTitle => {
                let res = match self.is_edit {
                    true => self
                        .repo
                        .software_titles
                        .update_software_title(&self.software_title),
                    false => self.repo.software_titles.add_software_title(
                        &self.software_title.name,
                        &self.software_title.franchise_id,
                    ),
                };

                match res {
                    Ok(_) => {
                        if let Err(err) = self.update_games() {
                            return Action::Error(err);
                        }
                        Action::GameSubmitted
                    }
                    Err(e) => Action::Error(e),
                }
            }
            Message::DeleteSoftwareTitle(id) => {
                match self.repo.software_titles.delete_software_title(&id) {
                    Ok(_) => {
                        self.software_titles.retain(|game| game.id != id);
                        Action::GameDeleted
                    }
                    Err(e) => Action::Error(e),
                }
            }
            Message::EditSoftwareTitle(id) => {
                match self.repo.software_titles.get_software_title(&id) {
                    Ok(game) => match game {
                        Some(game) => {
                            self.software_title = game;
                            self.is_edit = true;
                            Action::None
                        }
                        None => {
                            Action::Error(Error::DbError(format!("Game with id {} not found", &id)))
                        }
                    },
                    Err(e) => Action::Error(e),
                }
            }
            Message::NameChanged(name) => {
                self.software_title.name = name;
                Action::None
            }
            Message::Clear => {
                self.software_title = SoftwareTitle::default();
                Action::None
            }
            // TODO: handle error
            Message::SoftwareTitleListModelsLoaded(Ok(software_titles)) => {
                self.software_titles = software_titles;
                Action::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let back_button = button("Back").on_press(Message::Back);
        let name_input_field =
            text_input("Enter name", &self.software_title.name).on_input(Message::NameChanged);
        let main_buttons = row![
            button("Submit").on_press(Message::SubmitSoftwareTitle),
            button("Clear").on_press(Message::Clear)
        ];

        let games_list = self
            .software_titles
            .iter()
            .map(|game| {
                row![
                    text(&game.name).width(iced::Length::Fixed(300.0)),
                    button("Edit")
                        .on_press(Message::EditSoftwareTitle(game.id))
                        .width(iced::Length::Fixed(200.0)),
                    button("Delete")
                        .on_press_maybe(
                            game.can_delete
                                .then_some(Message::DeleteSoftwareTitle(game.id))
                        )
                        .width(iced::Length::Fixed(200.0))
                ]
                .into()
            })
            .collect::<Vec<Element<Message>>>();
        column![
            back_button,
            name_input_field,
            main_buttons,
            Column::with_children(games_list)
        ]
        .into()
    }
}
