use crate::model::Game;
use iced::widget::{button, column, text, Column};

pub struct ViewGame {
    game: Game,
}

#[derive(Debug, Clone)]
pub enum Message {
    GoToGames,
}

#[derive(Debug, Clone)]
pub enum Action {
    GoToGames,
}

impl ViewGame {
    pub fn new(game: Game) -> Self {
        Self { game }
    }

    pub fn title(&self) -> String {
        self.game.name.clone()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::GoToGames => Action::GoToGames,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let title = text(self.game.name.clone()).size(50);
        let releases_list = self
            .game
            .releases
            .iter()
            .map(|release| text(release.to_string()).into())
            .collect::<Vec<iced::Element<Message>>>();
        let back_button = button("Back").on_press(Message::GoToGames);
        column![title, Column::with_children(releases_list), back_button].into()
    }
}
