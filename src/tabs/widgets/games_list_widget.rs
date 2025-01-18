use bson::oid::ObjectId;
use iced::{
    widget::{button, row, text, Column},
    Element, Task,
};

use crate::{
    database_with_polo::DatabaseWithPolo,
    view_model::list_models::{get_games_as_list_model, GameListModel},
};

pub struct GamesList {
    pub games: Vec<GameListModel>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ViewGame(ObjectId),
}

impl GamesList {
    pub fn new() -> Self {
        let db = DatabaseWithPolo::get_instance();
        let games = get_games_as_list_model(db).unwrap_or_else(|err| {
            println!("Failed to get games list {:?}", err);
            vec![]
        });

        Self { games }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ViewGame(id) => {
                println!("Viewing game with id: {:?}", id);
            }
        }
        Task::none()
    }

    pub fn view(&self) -> iced::Element<Message> {
        let games = self.games.iter().map(|game| {
            row![
                text(game.name.clone()).width(iced::Length::Fixed(300.0)),
                button("View").on_press(Message::ViewGame(game.id)),
            ]
            .into()
        });
        Column::with_children(games.collect::<Vec<Element<Message>>>()).into()
    }
}
