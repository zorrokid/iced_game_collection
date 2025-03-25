use std::{path::PathBuf, sync::Arc};

use iced::{
    widget::{button, row, text},
    Task,
};

use crate::{
    database::repository_manager::RepositoryManager, error::Error, files::pick_folder,
    view_model::settings::Settings,
};

#[derive(Debug, Clone)]
pub struct SettingsWidget {
    settings: Arc<Settings>,
    repo: Arc<RepositoryManager>,
    is_locked: bool,
    collection_root_dir: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Submit,
    SelectFolder,
    FolderAdded(Result<PathBuf, Error>),
}

impl SettingsWidget {
    pub fn new(settings: Arc<Settings>, repo: Arc<RepositoryManager>) -> Result<Self, Error> {
        Ok(Self {
            is_locked: !settings.collection_root_dir.is_dir(),
            settings,
            repo,
            collection_root_dir: None,
        })
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Submit => {
                let db = DatabaseWithPolo::get_instance();
                match db.add_or_update_settings(&self.settings) {
                    Ok(_) => {
                        self.is_locked = true;
                    }
                    Err(err) => {
                        print!("Error adding settings: {:?}", err);
                    }
                }
                self.is_locked = true;
                Task::none()
            }
            Message::SelectFolder => Task::perform(pick_folder(), Message::FolderAdded),
            Message::FolderAdded(Ok(path)) => {
                self.settings.collection_root_dir = path.to_string_lossy().to_string();
                Task::none()
            }
            Message::FolderAdded(Err(err)) => {
                print!("Error adding folder: {:?}", err);
                Task::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let root_dir_str = self.settings.collection_root_dir.to_string_lossy();
        let collection_root_dir_input = text(root_dir_str.clone());

        let collection_root_dir_button = button("Collection root dir")
            .on_press_maybe((!self.is_locked).then_some(Message::SelectFolder));
        let save_button = button("Submit").on_press(Message::Submit);
        row![
            collection_root_dir_button,
            collection_root_dir_input,
            save_button
        ]
        .into()
    }
}
