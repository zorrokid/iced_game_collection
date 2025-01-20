use std::path::PathBuf;

use iced::{
    widget::{button, row, text},
    Task,
};

use crate::{
    database_with_polo::DatabaseWithPolo, error::Error, files::pick_folder, model::model::Settings,
};

#[derive(Debug, Clone)]
pub struct SettingsWidget {
    settings: Settings,
    is_locked: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Submit,
    SelectFolder,
    FolderAdded(Result<PathBuf, Error>),
}

impl SettingsWidget {
    pub fn new() -> Result<Self, Error> {
        let settings = DatabaseWithPolo::get_instance().get_settings()?;

        Ok(Self {
            is_locked: !settings.collection_root_dir.is_empty(),
            settings,
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
        let collection_root_dir_input = text(&self.settings.collection_root_dir);

        let collection_root_dir_button = button("Collection root dir")
            .on_press_maybe((!self.is_locked).then(|| Message::SelectFolder));
        let save_button = button("Submit").on_press(Message::Submit);
        row![
            collection_root_dir_button,
            collection_root_dir_input,
            save_button
        ]
        .into()
    }
}
