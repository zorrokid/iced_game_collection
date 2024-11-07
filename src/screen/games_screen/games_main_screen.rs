use iced::{
    widget::{button, column, row, text, Column},
    Element,
};

use crate::model::GameListModel;

#[derive(Debug, Clone)]
pub struct GamesMainScreen {
    pub games: Vec<GameListModel>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ViewGame(i32),
    GoHome,
}

pub enum Action {
    GoHome,
    ViewGame(i32),
}

impl GamesMainScreen {
    pub fn new() -> Self {
        let db = crate::database::Database::get_instance();
        let games = db.read().unwrap().to_game_list_model();
        Self { games }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ViewGame(id) => Action::ViewGame(id),
            Message::GoHome => Action::GoHome,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let games = self.games.iter().map(|game| {
            row![
                text(game.name.clone()).width(iced::Length::Fixed(300.0)),
                button("View").on_press(Message::ViewGame(game.id)),
            ]
            .into()
        });
        let games_list_with_container =
            Column::with_children(games.collect::<Vec<Element<Message>>>());
        let back_button = button("Back").on_press(Message::GoHome);
        column![back_button, games_list_with_container].into()
    }
}