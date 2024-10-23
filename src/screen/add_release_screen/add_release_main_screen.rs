use iced::widget::{button, column, text};

#[derive(Debug, Clone)]
pub struct AddReleaseMainScreen {}

#[derive(Debug, Clone)]
pub enum Message {
    ManageGames,
    ManageSystems,
    Back,
}

#[derive(Debug, Clone)]
pub enum Action {
    ManageGames,
    ManageSystems,
    Back,
}

impl AddReleaseMainScreen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ManageGames => Action::ManageGames,
            Message::ManageSystems => Action::ManageSystems,
            Message::Back => Action::Back,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let title = text("Add Release Main Screen");
        let back_button = button("Back").on_press(Message::Back);
        let manage_games_button = button("Manage Games")
            .width(iced::Length::Fixed(200.0))
            .on_press(Message::ManageGames);
        let manage_systems_button = button("Manage Systems")
            .width(iced::Length::Fixed(200.0))
            .on_press(Message::ManageSystems);
        column![
            title,
            back_button,
            manage_games_button,
            manage_systems_button,
        ]
        .into()
    }
}
