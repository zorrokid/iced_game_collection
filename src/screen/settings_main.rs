use crate::database::Database;
use crate::database_with_polo::DatabaseWithPolo;
use crate::error::Error;
use crate::model::model::Settings;
use crate::screen::settings_screen::settings_main_screen;

use super::settings_screen::SettingsScreen;
use iced::Task;

pub struct SettingsMain {
    screen: SettingsScreen,
    settings: Settings,
}

#[derive(Debug, Clone)]
pub enum Message {
    SettingsMainScreen(settings_main_screen::Message),
}

pub enum Action {
    Back,
    None,
    Run(Task<Message>),
}

impl SettingsMain {
    pub fn new() -> Result<Self, Error> {
        let db = DatabaseWithPolo::get_instance();

        let settings = db.get_settings()?;

        let collection_root_dir = settings.collection_root_dir.clone();
        Ok(Self {
            settings,
            screen: SettingsScreen::SettingsMainScreen(
                settings_main_screen::SettingsMainScreen::new(collection_root_dir),
            ),
        })
    }

    pub fn title(&self) -> String {
        "Settings".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::SettingsMainScreen(message) => {
                if let SettingsScreen::SettingsMainScreen(screen) = &mut self.screen {
                    match screen.update(message) {
                        settings_main_screen::Action::SetCollectionRootDir(dir) => {
                            self.settings.collection_root_dir = dir;
                            let db = DatabaseWithPolo::get_instance();
                            db.add_or_update_settings(&self.settings);

                            Action::None
                        }
                        settings_main_screen::Action::Back => Action::Back,
                        settings_main_screen::Action::None => Action::None,
                        settings_main_screen::Action::Run(task) => {
                            Action::Run(task.map(Message::SettingsMainScreen))
                        }
                    }
                } else {
                    Action::None
                }
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        match &self.screen {
            SettingsScreen::SettingsMainScreen(screen) => {
                screen.view().map(Message::SettingsMainScreen)
            }
        }
    }
}
