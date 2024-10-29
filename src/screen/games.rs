use crate::model::GameListModel;
use iced::widget::{button, column, row, text, Column};
use iced::Element;
pub struct Games {
    pub games: Vec<GameListModel>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ViewGame(i32),
    EditGame(i32),
    DeleteGame(i32),
    GoHome,
}

pub enum Action {
    GoHome,
    ViewGame(i32),
    EditGame(i32),
    GameDeleted,
}

impl Games {
    pub fn new() -> Self {
        let db = crate::database::Database::get_instance();
        let games = db.read().unwrap().to_game_list_model();
        Self { games }
    }

    pub fn title(&self) -> String {
        "Games".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ViewGame(id) => Action::ViewGame(id),
            Message::EditGame(id) => Action::EditGame(id),
            Message::DeleteGame(_id) => Action::GameDeleted,
            Message::GoHome => Action::GoHome,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let games = self.games.iter().map(|game| {
            row![
                text(game.name.clone()).width(iced::Length::Fixed(300.0)),
                button("View").on_press(Message::ViewGame(game.id)),
                button("Edit").on_press(Message::EditGame(game.id)),
                button("Delete").on_press_maybe(if game.can_delete {
                    Some(Message::DeleteGame(game.id))
                } else {
                    None
                }),
            ]
            .into()
        });
        let games_list_with_container =
            Column::with_children(games.collect::<Vec<Element<Message>>>());
        let back_button = button("Back").on_press(Message::GoHome);
        column![back_button, games_list_with_container].into()
    }
}
