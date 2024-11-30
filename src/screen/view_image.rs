use iced::widget::{button, column, image};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ViewImage {
    pub image_path: PathBuf,
}

#[derive(Debug, Clone)]
pub enum Message {
    Back,
}

pub enum Action {
    Back,
}

impl ViewImage {
    pub fn new(image_path: PathBuf) -> Self {
        Self { image_path }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Back => Action::Back,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let back_button = button("Back").on_press(Message::Back);
        let image = image(self.image_path.clone());
        column![back_button, image].into()
    }
}
