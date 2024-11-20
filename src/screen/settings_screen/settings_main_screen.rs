use crate::error::Error;
use iced::{
    widget::{button, column, row, text, text_input},
    Task,
};
use std::path::PathBuf;

use crate::files::pick_folder;

#[derive(Debug, Clone)]
pub struct SettingsMainScreen {
    collection_root_dir: String,
    is_locked: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Submit,
    Back,
    SelectFolder,
    FolderAdded(Result<PathBuf, Error>),
}

pub enum Action {
    Back,
    SetCollectionRootDir(String),
    None,
    Run(Task<Message>),
}

impl SettingsMainScreen {
    pub fn new(collection_root_dir: String) -> Self {
        Self {
            is_locked: !collection_root_dir.clone().is_empty(),
            collection_root_dir,
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Submit => {
                self.is_locked = true;
                Action::SetCollectionRootDir(self.collection_root_dir.clone())
            }
            Message::Back => Action::Back,
            Message::SelectFolder => {
                Action::Run(Task::perform(pick_folder(), Message::FolderAdded))
            }
            Message::FolderAdded(Ok(path)) => {
                self.collection_root_dir = path.to_string_lossy().to_string();
                Action::None
            }
            Message::FolderAdded(Err(err)) => {
                print!("Error adding folder: {:?}", err);
                Action::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let collection_root_dir_input = text(&self.collection_root_dir);

        let collection_root_dir_button =
            button("Collection root dir").on_press_maybe(match self.is_locked {
                true => None,
                false => Some(Message::SelectFolder),
            });
        let save_button = button("Submit").on_press(Message::Submit);

        let root_dir_row = row![
            collection_root_dir_button,
            collection_root_dir_input,
            save_button
        ];

        let back_button = button("Back").on_press(Message::Back);

        column![back_button, root_dir_row].into()
    }
}
