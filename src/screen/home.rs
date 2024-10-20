use iced::widget::{button, column};

pub struct Home {}

#[derive(Debug, Clone)]
pub enum Message {
    ViewGames,
    AddGame,
    ManageSystems,
    ManageEmulators,
    Exit,
}

pub enum Action {
    ViewGames,
    AddGame,
    ManageSystems,
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
            Message::AddGame => Action::AddGame,
            Message::ManageEmulators => Action::ManageEmulators,
            Message::Exit => Action::Exit,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let view_games_button = button("View Games")
            .width(iced::Length::Fixed(200.0))
            .on_press(Message::ViewGames);
        let add_game_button = button("Add Game")
            .width(iced::Length::Fixed(200.0))
            .on_press(Message::AddGame);
        let manage_systems_button = button("Manage systems")
            .width(iced::Length::Fixed(200.0))
            .on_press(Message::ManageSystems);
        let manage_emulators_button = button("Manage emulators")
            .width(iced::Length::Fixed(200.0))
            .on_press(Message::ManageEmulators);
        let exit_button = button("Save & Exit")
            .width(iced::Length::Fixed(200.0))
            .on_press(Message::Exit);

        column![
            view_games_button,
            add_game_button,
            manage_systems_button,
            manage_emulators_button,
            exit_button
        ]
        .into()
    }
}
