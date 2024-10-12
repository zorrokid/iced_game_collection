use crate::model::Game;
use iced::widget::{button, column, text, Column};

pub struct ViewGame {
    game: Game,
}

#[derive(Debug, Clone)]
pub enum Message {
    GoHome,
}

#[derive(Debug, Clone)]
pub enum Action {
    GoHome,
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
            Message::GoHome => Action::GoHome,
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
        let home_button = button("Home").on_press(Message::GoHome);
        column![title, Column::with_children(releases_list), home_button].into()
    }
}