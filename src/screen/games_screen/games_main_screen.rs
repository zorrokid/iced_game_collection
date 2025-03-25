use std::sync::Arc;

use iced::{
    widget::{button, column, row, text, Column},
    Element, Task,
};

use crate::{
    database::{
        database_error::DatabaseError, repository_manager::RepositoryManager,
        software_title_repository::SoftwareTitleWriteRepository,
    },
    error::Error,
    service::view_model_service::ViewModelService,
    view_model::list_models::SoftwareTitleListModel,
};

#[derive(Debug, Clone)]
pub struct GamesMainScreen {
    repo: Arc<RepositoryManager>,
    view_model_service: Arc<ViewModelService>,
    software_titles: Vec<SoftwareTitleListModel>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ViewGame(i64),
    DeleteGame(i64),
    GoHome,
    GameDeleted(Result<i64, DatabaseError>),
    SoftwareTitleListModelsLoaded(Result<Vec<SoftwareTitleListModel>, DatabaseError>),
}

pub enum Action {
    GoHome,
    ViewGame(i64),
    None,
    Error(Error),
    Run(Task<Message>),
}

impl GamesMainScreen {
    pub fn new(
        repo: Arc<RepositoryManager>,
        view_model_service: Arc<ViewModelService>,
    ) -> (Self, Task<Message>) {
        let service = Arc::clone(&view_model_service);
        (
            Self {
                software_titles: vec![],
                repo,
                view_model_service,
            },
            Task::perform(
                async move { service.get_software_title_list_models().await },
                Message::SoftwareTitleListModelsLoaded,
            ),
        )
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ViewGame(id) => Action::ViewGame(id),
            Message::GoHome => Action::GoHome,
            Message::DeleteGame(id) => {
                let repo = Arc::clone(&self.repo);
                Action::Run(Task::perform(
                    async move { repo.software_titles().delete_software_title(id).await },
                    Message::GameDeleted,
                ))
            }
            Message::GameDeleted(result) => match result {
                Ok(id) => {
                    self.software_titles.retain(|game| game.id != id);
                    Action::None
                }
                Err(e) => {
                    eprintln!("Failed to delete game {:?}", e);
                    Action::Error(Error::DbError(e.to_string()))
                }
            },
            Message::SoftwareTitleListModelsLoaded(result) => match result {
                Ok(games) => {
                    self.software_titles = games;
                    Action::None
                }
                Err(e) => {
                    eprintln!("Failed to load software titles {:?}", e);
                    Action::Error(Error::DbError(e.to_string()))
                }
            },
        }
    }

    pub fn view(&self) -> Element<Message> {
        let games = self.software_titles.iter().map(|game| {
            row![
                text(game.name.clone()).width(iced::Length::Fixed(300.0)),
                button("View").on_press(Message::ViewGame(game.id)),
                button("Delete")
                    .on_press_maybe(game.can_delete.then_some(Message::DeleteGame(game.id)))
            ]
            .into()
        });
        let games_list_with_container =
            Column::with_children(games.collect::<Vec<Element<Message>>>());
        let back_button = button("Back").on_press(Message::GoHome);
        column![back_button, games_list_with_container].into()
    }
}
