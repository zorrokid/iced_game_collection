use iced::widget::{button, column, text};

#[derive(Debug, Clone)]
pub struct ManageSystemsScreen {}

#[derive(Debug, Clone)]
pub enum Message {
    Back,
}

#[derive(Debug, Clone)]
pub enum Action {
    Back,
}

impl ManageSystemsScreen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Back => Action::Back,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let back_button = button("Back").on_press(Message::Back);
        let title = text("Manage Systems Screen");
        column![title, back_button].into()
    }
}
