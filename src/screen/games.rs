use crate::model::Game;
use iced::widget::{button, column, row, text, Column};
use iced::Element;
pub struct Games {
    pub games: Vec<Game>,
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
    None,
}

impl Games {
    pub fn new(games: Vec<Game>) -> Self {
        Self { games }
    }

    pub fn title(&self) -> String {
        "Games".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ViewGame(id) => Action::ViewGame(id),
            Message::EditGame(id) => Action::None,
            Message::DeleteGame(id) => Action::None,
            Message::GoHome => Action::GoHome,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let games = self.games.iter().map(|game| {
            row![
                text(game.id.to_string()),
                text(game.name.clone()),
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
