use iced::widget::{button, column, text};

use crate::{
    database::Database, database_with_polo::DatabaseWithPolo, error::Error, model::model::Settings,
};

pub struct Home {
    settings: Settings,
}

#[derive(Debug, Clone)]
pub enum Message {
    ViewGames,
    AddRelease,
    ManageSystems,
    ManageGames,
    ManageEmulators,
    ManageSettings,
    Exit,
}

pub enum Action {
    ViewGames,
    AddRelease,
    ManageSystems,
    ManageGames,
    ManageEmulators,
    ManageSettings,
    Exit,
}

impl Home {
    pub fn new() -> Result<Self, Error> {
        let db = DatabaseWithPolo::get_instance();
        let settings = db.get_settings()?;
        Ok(Self { settings })
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
            Message::ManageSettings => Action::ManageSettings,
            Message::Exit => Action::Exit,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let settings_button = button("Settings")
            .width(iced::Length::Fixed(200.0))
            .on_press(Message::ManageSettings);
        let exit_button = button("Save & Exit")
            .width(iced::Length::Fixed(200.0))
            .on_press(Message::Exit);

        if self.settings.collection_root_dir.is_empty() {
            return column![
                text("Welcome to Iced Game Collection!"),
                text("Please set your collection root directory in settings."),
                settings_button,
                exit_button
            ]
            .into();
        }
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
        column![
            view_games_button,
            add_release_button,
            manage_systems_button,
            manage_games_button,
            manage_emulators_button,
            settings_button,
            exit_button
        ]
        .into()
    }
}
