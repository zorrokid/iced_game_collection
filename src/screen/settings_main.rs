use std::sync::Arc;

use crate::database::repository_manager::RepositoryManager;
use crate::database::setting_repository::SettingReadRepository as _;
use crate::error::Error;
use crate::model::settings::SettingName;
use crate::screen::settings_screen::settings_main_screen;

use super::settings_screen::SettingsScreen;
use iced::Task;

pub struct SettingsMain {
    repo: Arc<RepositoryManager>,
    screen: SettingsScreen,
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
    pub fn new(repo: Arc<RepositoryManager>) -> Result<Self, Error> {
        let collection_root_dir = repo
            .settings()
            .get_setting(SettingName::CollectionRootDir)?;
        Ok(Self {
            screen: SettingsScreen::SettingsMainScreen(
                settings_main_screen::SettingsMainScreen::new(collection_root_dir),
            ),
            repo,
        })
    }

    pub fn title(&self) -> String {
        "Settings".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::SettingsMainScreen(message) => {
                let SettingsScreen::SettingsMainScreen(screen) = &mut self.screen;
                match screen.update(message) {
                    settings_main_screen::Action::SetCollectionRootDir(dir) => {
                        self.settings.collection_root_dir = dir;
                        if let Err(err) = self.repo.settings.add_or_update_setting(
                            SettingName::CollectionRootDir,
                            &self.settings.collection_root_dir,
                        ) {
                            // TODO: Show error
                            eprintln!("Failed to update settings {:?}", err);
                        }

                        Action::None
                    }
                    settings_main_screen::Action::Back => Action::Back,
                    settings_main_screen::Action::None => Action::None,
                    settings_main_screen::Action::Run(task) => {
                        Action::Run(task.map(Message::SettingsMainScreen))
                    }
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
