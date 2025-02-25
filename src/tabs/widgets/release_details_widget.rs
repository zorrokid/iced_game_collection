use crate::database::database_with_polo::DatabaseWithPolo;
use crate::model::model::HasOid;
use crate::repository::SettingsReadRepository;
use crate::util::file_path_builder::FilePathBuilder;
use crate::util::image::get_thumbnail_path;
use crate::view_model::release_view_model::ReleaseViewModel;
use crate::{
    model::{collection_file::CollectionFileType, model::Settings},
    view_model::release_view_model::get_release_view_model,
};
use bson::oid::ObjectId;
use iced::widget::{button, image, Column};
use iced::Element;
use iced::{
    widget::{column, row, text},
    Task,
};
use std::path::PathBuf;
use std::vec;

use super::emulator_files_list_widget::{self, EmulatorFilesList};

pub struct ReleaseDetails {
    release: Option<ReleaseViewModel>,
    settings: Settings,
    file_path_builder: FilePathBuilder,
    emulator_files_list: EmulatorFilesList,
}

#[derive(Debug, Clone)]
pub enum Message {
    ReleaseSelected(ObjectId),
    ViewImage(PathBuf),
    EmulatorFilesList(emulator_files_list_widget::Message),
}

pub enum Action {
    Run(Task<Message>),
    ImageSelected(PathBuf),
    None,
}

impl ReleaseDetails {
    pub fn new() -> Self {
        let db = DatabaseWithPolo::get_instance();
        let settings = db.get_settings().unwrap_or_else(|err| {
            println!("Failed to get settings {:?}", err);
            Settings::default()
        });
        let file_path_builder = FilePathBuilder::new(settings.collection_root_dir.clone());

        Self {
            release: None,
            settings,
            file_path_builder,
            emulator_files_list: EmulatorFilesList::new(None, vec![]),
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ReleaseSelected(release_id) => {
                let db = DatabaseWithPolo::get_instance();
                let release = get_release_view_model(&release_id, db).unwrap_or_else(|err| {
                    println!("Failed to get release {:?}", err);
                    None
                });
                self.release = release;
                if let Some(release) = &self.release {
                    self.emulator_files_list
                        .update(emulator_files_list_widget::Message::SetFiles(
                            release.files.clone(),
                            release.system.id(),
                        ));
                }
            }
            Message::ViewImage(path) => return Action::ImageSelected(path),
            Message::EmulatorFilesList(message) => {
                match self.emulator_files_list.update(message) {
                    emulator_files_list_widget::Action::Run(task) => {
                        return Action::Run(task.map(Message::EmulatorFilesList));
                    }
                    emulator_files_list_widget::Action::RemoveFileReference(_id) => {
                        // TODO
                    }
                    emulator_files_list_widget::Action::None => {}
                };
            }
        }
        Action::None
    }

    pub fn view(&self) -> iced::Element<Message> {
        let selected_games_list = self.create_selected_games_list();
        let emulator_files_list = self
            .emulator_files_list
            .view()
            .map(Message::EmulatorFilesList);
        let scan_files_list = self.create_files_list(&CollectionFileType::CoverScan);
        let screenshot_files_list = self.create_files_list(&CollectionFileType::Screenshot);

        column![
            selected_games_list,
            emulator_files_list,
            scan_files_list,
            screenshot_files_list
        ]
        .into()
    }

    fn create_selected_games_list(&self) -> Element<Message> {
        let selected_games_title = text("Games in release:");
        if let Some(release) = &self.release {
            let game_names = release
                .games
                .iter()
                .map(|game| text(game.name.clone()).into())
                .collect::<Vec<Element<Message>>>();

            return column![selected_games_title, Column::with_children(game_names)].into();
        }

        selected_games_title.into()
    }

    // TODO: use files_list_widget
    fn create_files_list(&self, file_type: &CollectionFileType) -> Element<Message> {
        if let Some(release) = &self.release {
            let scan_files_list = release
                .files
                .iter()
                .filter(|f| f.collection_file_type == *file_type)
                .filter_map(|file| {
                    if let Ok(thumb_path) = get_thumbnail_path(
                        file,
                        &self.settings.collection_root_dir,
                        &release.system.id(),
                    ) {
                        if let Ok(file_path) = self
                            .file_path_builder
                            .build_file_path(&release.system.id(), file)
                        {
                            let image = image(thumb_path);
                            let view_image_button =
                                button(image).on_press(Message::ViewImage(file_path));
                            return Some(row![view_image_button].into());
                        }
                    }

                    None
                })
                .collect::<Vec<iced::Element<Message>>>();

            Column::with_children(scan_files_list).into()
        } else {
            Column::new().into()
        }
    }
}
