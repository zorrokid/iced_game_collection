use iced::{widget::text, Task};

use super::widgets::add_game_widget;

pub struct AddReleaseTab {
    add_game_widget: add_game_widget::AddGame,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddGame(add_game_widget::Message),
}

impl AddReleaseTab {
    pub fn new() -> Self {
        Self {
            add_game_widget: add_game_widget::AddGame::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AddGame(message) => {
                self.add_game_widget.update(message);
                Task::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        self.add_game_widget.view().map(Message::AddGame)
    }
}
