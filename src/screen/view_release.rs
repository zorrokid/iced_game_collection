use std::path::PathBuf;
use std::{collections::HashMap, env, vec};

use crate::emulator_runner::EmulatorRunOptions;
use crate::error::Error;
use crate::model::model::HasOid;
use crate::model::{
    collection_file::{CollectionFileType, GetFileExtensions},
    model::{Emulator, Settings, System},
};
use crate::util::file_path_builder::FilePathBuilder;
use crate::util::image::get_thumbnail_path;
use crate::view_model::release_view_model::{get_release_view_model, ReleaseViewModel};
use bson::oid::ObjectId;
use iced::widget::{button, column, image, pick_list, row, text, Column};
use iced::{Element, Task};

#[derive(Debug, Clone)]
pub struct ViewRelease {
    release: ReleaseViewModel,
    selected_file: HashMap<ObjectId, String>,
    emulators: Vec<Emulator>,
    settings: Settings,
    file_path_builder: FilePathBuilder,
}

#[derive(Debug, Clone)]
pub enum Message {
    Back,
    RunWithEmulator(Emulator, String, CollectionFileType),
    ViewImage(PathBuf),
    FileSelected(ObjectId, String),
}

pub enum Action {
    Back,
    None,
    Run(Task<Message>),
    RunWithEmulator(EmulatorRunOptions),
    ViewImage(PathBuf),
    Error(Error),
}

impl ViewRelease {
    pub fn new(release_id: ObjectId) -> Result<Self, Error> {
        let db = crate::database_with_polo::DatabaseWithPolo::get_instance();
        let release = get_release_view_model(&release_id, db)?;
        // TODO: get emulators for the system of the release
        let emulators = db.get_emulators()?;
        let settings = db.get_settings()?;
        let file_path_builder = FilePathBuilder::new(settings.collection_root_dir.clone());

        match release {
            None => Err(Error::NotFound(format!(
                "Release with id {} not found",
                release_id
            ))),
            Some(release) => Ok(Self {
                release,
                selected_file: HashMap::new(),
                emulators,
                settings,
                file_path_builder,
            }),
        }
    }

    pub fn title(&self) -> String {
        self.release.name.clone()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Back => Action::Back,
            Message::RunWithEmulator(emulator, selected_file_name, collection_file_type) => {
                let system = &self.release.system;
                let options = EmulatorRunOptions {
                    emulator,
                    files: self.release.files.clone(),
                    selected_file_name: selected_file_name,
                    source_path: self
                        .file_path_builder
                        .build_target_directory(system, &collection_file_type),
                    target_path: env::temp_dir(),
                };
                Action::RunWithEmulator(options)
            }
            Message::ViewImage(file_path) => Action::ViewImage(file_path),
            Message::FileSelected(id, file) => {
                self.selected_file.insert(id, file);
                Action::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let back_button = button("Back").on_press(Message::Back);
        let selected_games_list = self.create_selected_games_list();

        let emulator_files_list = self.create_emulator_files_list(&self.release.system);
        let scan_files_list = self.create_scan_files_list();

        column![
            back_button,
            selected_games_list,
            emulator_files_list,
            scan_files_list,
        ]
        .into()
    }

    fn create_selected_games_list(&self) -> Element<Message> {
        let selected_games_title = text("Games in release:");

        let selected_games_list = self
            .release
            .games
            .iter()
            .map(|game| text(&game.name).into())
            .collect::<Vec<Element<Message>>>();

        column![
            selected_games_title,
            Column::with_children(selected_games_list),
        ]
        .into()
    }

    fn create_scan_files_list(&self) -> Element<Message> {
        let scan_files_list = self
            .release
            .files
            .iter()
            .filter(|f| f.collection_file_type == CollectionFileType::CoverScan)
            .filter_map(|file| {
                if let Ok(thumb_path) =
                    get_thumbnail_path(file, &self.settings, &self.release.system)
                {
                    if let Ok(file_path) = self
                        .file_path_builder
                        .build_file_path(&self.release.system, file)
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
    }

    fn create_emulator_files_list(&self, selected_system: &System) -> Element<Message> {
        let emulators_for_system = self
            .emulators
            .iter()
            .filter(|emulator| {
                emulator
                    .system_id
                    .map_or(false, |system_id| system_id == selected_system.id())
            })
            .collect::<Vec<&Emulator>>();

        // TODO: get file types supported by emulators for the selected system

        let files_list = self
            .release
            .files
            .iter()
            .filter(|f| {
                // TODO: emulator should know it's supported file types and we would filter by the file types supported by emulator
                f.collection_file_type == CollectionFileType::Rom
                    || f.collection_file_type == CollectionFileType::DiskImage
                    || f.collection_file_type == CollectionFileType::TapeImage
            })
            .map(|file| {
                let container_filename = text(file.to_string());
                let content_files: Vec<String> = if let Some(files) = &file.files {
                    files.iter().map(|file| file.name.clone()).collect()
                } else {
                    vec![]
                };
                let file_picker = pick_list(
                    content_files,
                    if self.selected_file.contains_key(&file.id()) {
                        Some(self.selected_file.get(&file.id()).unwrap())
                    } else {
                        None
                    },
                    move |selected_file_name| {
                        Message::FileSelected(file.id().clone(), selected_file_name)
                    },
                );
                let emulator_buttons = emulators_for_system
                    .iter()
                    .filter(|e| {
                        e.supported_file_type_extensions.is_empty()
                            || e.supported_file_type_extensions.contains(
                                &file
                                    .original_file_name
                                    .split('.')
                                    .last()
                                    .unwrap()
                                    .to_string(),
                            )
                            || file.get_file_extensions().into_iter().any(|extension| {
                                e.supported_file_type_extensions.contains(&extension)
                            })
                    })
                    .map(|emulator| {
                        button(emulator.name.as_str())
                            .on_press_maybe({
                                let selected_file = self.selected_file.get(&file.id());
                                match (selected_file, emulator.extract_files) {
                                    (Some(file_name), true) => Some(Message::RunWithEmulator(
                                        (*emulator).clone(),
                                        file_name.clone(),
                                        file.collection_file_type.clone(),
                                    )),
                                    (_, false) => Some(Message::RunWithEmulator(
                                        (*emulator).clone(),
                                        file.clone().original_file_name,
                                        file.collection_file_type.clone(),
                                    )),
                                    (_, _) => None,
                                }
                            })
                            .into()
                    })
                    .collect::<Vec<iced::Element<Message>>>();
                row![
                    container_filename,
                    file_picker,
                    Column::with_children(emulator_buttons)
                ]
                .into()
            })
            .collect::<Vec<iced::Element<Message>>>();
        Column::with_children(files_list).into()
    }
}
