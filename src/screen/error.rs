use iced::widget::{button, column, text};

pub struct Error {
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    GoHome,
}

pub enum Action {
    GoHome,
}

impl Error {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn title(&self) -> String {
        "Error".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::GoHome => Action::GoHome,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let message = text!("{}", &self.message);
        let go_home_button = button("Go Home").on_press(Message::GoHome);
        column![go_home_button, message].into()
    }
}
