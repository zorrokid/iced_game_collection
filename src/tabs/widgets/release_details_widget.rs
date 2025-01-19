use crate::model::model::HasOid;
use crate::util::file_path_builder::FilePathBuilder;
use crate::util::image::get_thumbnail_path;
use crate::view_model::release_view_model::ReleaseViewModel;
use crate::{
    model::{
        collection_file::{CollectionFileType, GetFileExtensions},
        model::{Emulator, Release, Settings, System},
    },
    view_model::release_view_model::get_release_view_model,
};
use bson::oid::ObjectId;
use iced::widget::{button, image, pick_list, Column};
use iced::Element;
use iced::{
    widget::{column, row, text},
    Task,
};
use std::path::PathBuf;
use std::{collections::HashMap, env, vec};

pub struct ReleaseDetails {
    release: Option<ReleaseViewModel>,
    selected_file: HashMap<ObjectId, String>,
    emulators: Vec<Emulator>,
    settings: Settings,
    file_path_builder: FilePathBuilder,
}

#[derive(Debug, Clone)]
pub enum Message {
    ReleaseSelected(ObjectId),
    ViewImage(PathBuf),
    RunWithEmulator(Emulator, String, CollectionFileType),
    FileSelected(ObjectId, String),
}

pub enum Action {
    None,
}

impl ReleaseDetails {
    pub fn new() -> Self {
        let db = crate::database_with_polo::DatabaseWithPolo::get_instance();
        let emulators = db.get_emulators().unwrap_or_else(|err| {
            println!("Failed to get emulators {:?}", err);
            vec![]
        });
        let settings = db.get_settings().unwrap_or_else(|err| {
            println!("Failed to get settings {:?}", err);
            Settings::default()
        });
        let file_path_builder = FilePathBuilder::new(settings.collection_root_dir.clone());

        Self {
            release: None,
            selected_file: HashMap::new(),
            emulators,
            settings,
            file_path_builder,
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ReleaseSelected(release_id) => {
                let db = crate::database_with_polo::DatabaseWithPolo::get_instance();
                let release = get_release_view_model(&release_id, db).unwrap_or_else(|err| {
                    println!("Failed to get release {:?}", err);
                    None
                });
                self.release = release;
            }
            Message::ViewImage(path) => {
                // TODO
            }
            Message::FileSelected(id, file) => {
                // TODO
            }
            Message::RunWithEmulator(emulator, selected_file_name, selcted_file_type) => {
                // TODO
            }
        }
        // TODO: handle ReleaseSelected message
        // Update fields here
        Action::None
    }

    pub fn view(&self) -> iced::Element<Message> {
        let selected_games_list = self.create_selected_games_list();
        let emulator_files_list = self.create_emulator_files_list();
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

    fn create_files_list(&self, file_type: &CollectionFileType) -> Element<Message> {
        if let Some(release) = &self.release {
            let scan_files_list = release
                .files
                .iter()
                .filter(|f| f.collection_file_type == *file_type)
                .filter_map(|file| {
                    if let Ok(thumb_path) =
                        get_thumbnail_path(file, &self.settings, &release.system)
                    {
                        if let Ok(file_path) = self
                            .file_path_builder
                            .build_file_path(&release.system, file)
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

    fn create_emulator_files_list(&self) -> Element<Message> {
        if let Some(release) = &self.release {
            let emulators_for_system = self
                .emulators
                .iter()
                .filter(|emulator| {
                    emulator
                        .system_id
                        .map_or(false, |system_id| system_id == release.system.id())
                })
                .collect::<Vec<&Emulator>>();

            // TODO: get file types supported by emulators for the selected system

            let files_list = release
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
        } else {
            Column::new().into()
        }
    }
}
