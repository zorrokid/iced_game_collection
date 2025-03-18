use std::sync::Arc;

use bson::oid::ObjectId;
use iced::{
    widget::{button, column, row, text, Column},
    Element,
};

use crate::{
    database::{
        repository_manager::RepositoryManager,
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
    ViewGame(ObjectId),
    DeleteGame(ObjectId),
    GoHome,
}

pub enum Action {
    GoHome,
    ViewGame(ObjectId),
    None,
    Error(Error),
}

impl GamesMainScreen {
    pub fn new(
        repo: Arc<RepositoryManager>,
        view_model_service: Arc<ViewModelService>,
    ) -> Result<Self, Error> {
        let games = view_model_service.get_software_title_list_models()?;
        Ok(Self {
            software_titles: games,
            repo,
            view_model_service,
        })
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ViewGame(id) => Action::ViewGame(id),
            Message::GoHome => Action::GoHome,
            Message::DeleteGame(id) => match self.repo.software_titles.delete_software_title(&id) {
                Ok(_) => {
                    self.software_titles.retain(|game| game.id != id);
                    Action::None
                }
                Err(e) => Action::Error(e),
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
