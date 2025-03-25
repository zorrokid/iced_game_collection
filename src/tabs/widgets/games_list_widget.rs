use std::sync::Arc;

use iced::{
    widget::{button, column, row, text, Column},
    Element, Task,
};

use crate::{
    database::database_error::DatabaseError, service::view_model_service::ViewModelService,
    view_model::list_models::SoftwareTitleListModel,
};

pub struct GamesList {
    pub software_titles: Vec<SoftwareTitleListModel>,
    pub selected_game: Option<i64>,
    pub view_model_service: Arc<ViewModelService>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ViewGame(i64),
    Refresh,
    SoftwareTitleListModelsLoaded(Result<Vec<SoftwareTitleListModel>, DatabaseError>),
}

pub enum Action {
    ViewGame(i64),
    None,
}

impl GamesList {
    pub fn new(view_model_service: Arc<ViewModelService>) -> (Self, Task<Message>) {
        let service_clone = Arc::clone(&view_model_service);

        (
            Self {
                software_titles: vec![],
                selected_game: None,
                view_model_service,
            },
            Task::perform(
                async move { service_clone.get_software_title_list_models().await },
                Message::SoftwareTitleListModelsLoaded,
            ),
        )
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ViewGame(id) => {
                self.selected_game = Some(id);
                println!("ViewGame message received with id: {:?}", id);
                Action::ViewGame(id)
            }
            Message::Refresh => {
                let db = DatabaseWithPolo::get_instance();
                self.software_titles = get_games_as_list_model(db).unwrap_or_else(|err| {
                    println!("Failed to get games list {:?}", err);
                    vec![]
                });
                Action::None
            }
            Message::SoftwareTitleListModelsLoaded(Ok(software_titles)) => {
                self.software_titles = software_titles;
                Action::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let games = self.software_titles.iter().map(|game| {
            row![
                text(game.name.clone()).width(iced::Length::Fixed(300.0)),
                button("View").on_press(Message::ViewGame(game.id)),
            ]
            .into()
        });
        let games_list = Column::with_children(games.collect::<Vec<Element<Message>>>());
        column![games_list].into()
    }
}
