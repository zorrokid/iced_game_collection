use bson::oid::ObjectId;
use iced::{
    widget::{button, column, row, text, Column},
    Element,
};

use crate::{
    database_with_polo::DatabaseWithPolo,
    error::Error,
    view_model::list_models::{get_games_as_list_model, GameListModel},
};

#[derive(Debug, Clone)]
pub struct GamesMainScreen {
    pub games: Vec<GameListModel>,
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
    pub fn new() -> Result<Self, Error> {
        let db = DatabaseWithPolo::get_instance();
        let games = get_games_as_list_model(db)?;
        Ok(Self { games })
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ViewGame(id) => Action::ViewGame(id),
            Message::GoHome => Action::GoHome,
            Message::DeleteGame(id) => {
                let db = DatabaseWithPolo::get_instance();
                match db.delete_game(&id) {
                    Ok(_) => {
                        self.games.retain(|game| game.id != id);
                        Action::None
                    }
                    Err(e) => Action::Error(e),
                }
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let games = self.games.iter().map(|game| {
            row![
                text(game.name.clone()).width(iced::Length::Fixed(300.0)),
                button("View").on_press(Message::ViewGame(game.id)),
                button("Delete")
                    .on_press_maybe(game.can_delete.then(|| Message::DeleteGame(game.id)))
            ]
            .into()
        });
        let games_list_with_container =
            Column::with_children(games.collect::<Vec<Element<Message>>>());
        let back_button = button("Back").on_press(Message::GoHome);
        column![back_button, games_list_with_container].into()
    }
}
