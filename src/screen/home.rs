use iced::widget::{button, column};

pub struct Home {}

#[derive(Debug, Clone)]
pub enum Message {
    ViewGames,
    AddRelease,
    ManageSystems,
    ManageGames,
    ManageEmulators,
    Exit,
}

pub enum Action {
    ViewGames,
    AddRelease,
    ManageSystems,
    ManageGames,
    ManageEmulators,
    Exit,
}

impl Home {
    pub fn new() -> Self {
        Self {}
    }

    pub fn title(&self) -> String {
        "Iced Game Collection".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ViewGames => Action::ViewGames,
            Message::ManageSystems => Action::ManageSystems,
            Message::ManageGames => Action::ManageGames,
            Message::AddRelease => Action::AddRelease,
            Message::ManageEmulators => Action::ManageEmulators,
            Message::Exit => Action::Exit,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let view_games_button = button("View Games")
            .width(iced::Length::Fixed(200.0))
            .on_press(Message::ViewGames);
        let add_release_button = button("Add release")
            .width(iced::Length::Fixed(200.0))
            .on_press(Message::AddRelease);
        let manage_systems_button = button("Manage systems")
            .width(iced::Length::Fixed(200.0))
            .on_press(Message::ManageSystems);
        let manage_games_button = button("Manage games")
            .width(iced::Length::Fixed(200.0))
            .on_press(Message::ManageGames);
        let manage_emulators_button = button("Manage emulators")
            .width(iced::Length::Fixed(200.0))
            .on_press(Message::ManageEmulators);
        let exit_button = button("Save & Exit")
            .width(iced::Length::Fixed(200.0))
            .on_press(Message::Exit);

        column![
            view_games_button,
            add_release_button,
            manage_systems_button,
            manage_games_button,
            manage_emulators_button,
            exit_button
        ]
        .into()
    }
}
