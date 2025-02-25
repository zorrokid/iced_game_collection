use bson::oid::ObjectId;
use iced::{
    widget::{button, column, row, text, Column},
    Element,
};

use crate::{
    database::database_with_polo::DatabaseWithPolo,
    view_model::list_models::{get_games_as_list_model, GameListModel},
};

pub struct GamesList {
    pub games: Vec<GameListModel>,
    pub selected_game: Option<ObjectId>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ViewGame(ObjectId),
    Refresh,
}

pub enum Action {
    ViewGame(ObjectId),
    None,
}

impl GamesList {
    pub fn new() -> Self {
        let db = DatabaseWithPolo::get_instance();
        let games = get_games_as_list_model(db).unwrap_or_else(|err| {
            println!("Failed to get games list {:?}", err);
            vec![]
        });

        Self {
            games,
            selected_game: None,
        }
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
                self.games = get_games_as_list_model(db).unwrap_or_else(|err| {
                    println!("Failed to get games list {:?}", err);
                    vec![]
                });
                Action::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let games = self.games.iter().map(|game| {
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
