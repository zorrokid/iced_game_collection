use iced::{widget::text, Element};

#[derive(Debug, Clone)]
pub struct GamesMainScreen {}

#[derive(Debug, Clone)]
pub enum Message {
    None,
}

pub enum Action {
    None,
}

impl GamesMainScreen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            _ => Action::None,
        }
    }

    pub fn view(&self) -> Element<Message> {
        text("Games").into()
    }
}
