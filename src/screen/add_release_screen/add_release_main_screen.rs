use std::path::PathBuf;
use std::{collections::HashMap, vec};

use crate::database_with_polo::DatabaseWithPolo;
use crate::error::Error;
use crate::files::{copy_file, delete_file, pick_file, PickedFile};
use crate::model::model::HasOid;
use crate::model::{
    collection_file::{CollectionFile, CollectionFileType},
    model::{Game, Release, Settings, System},
};
use crate::repository::repository::CollectionFilesReadRepository;
use crate::util::file_path_builder::FilePathBuilder;
use crate::util::image::get_thumbnail_path;
use bson::oid::ObjectId;
use iced::widget::{button, column, image, pick_list, row, text, text_input, Column};
use iced::{Element, Task};

#[derive(Debug, Clone)]
pub struct AddReleaseMainScreen {
    games: Vec<Game>,
    selected_game: Option<Game>,
    release: Release,
    systems: Vec<System>,
    selected_file: HashMap<ObjectId, String>,
    selected_file_type: Option<CollectionFileType>,
    settings: Settings,
    file_path_builder: FilePathBuilder,
    files: Vec<CollectionFile>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ManageGames,
    ManageSystems,
    Back,
    GameSelected(Game),
    NameChanged(String),
    SystemSelected(System),
    SelectFile,
    FilePicked(Result<PickedFile, Error>),
    Submit,
    Clear,
    FileSelected(ObjectId, String),
    CollectionFileTypeSelected(CollectionFileType),
    ViewImage(PathBuf),
    FileCopied(Result<ObjectId, Error>),
    DeleteFile(ObjectId),
    FileDeleted(Result<(), Error>, ObjectId),
    Save,
}

pub enum Action {
    ManageGames,
    ManageSystems,
    Back,
    GameSelected(Game),
    NameChanged(String),
    None,
    SystemSelected(System),
    Run(Task<Message>),
    AddFile(ObjectId),
    Submit,
    Clear,
    ViewImage(PathBuf),
    Error(Error),
    DeleteFile(ObjectId),
    Save,
}

// TODO: add ViewReleaseScreen just for viewing release and using the view model
// Cannot use view model here because when adding a release, it doesn't have id yet
// TODO: split files management to another sub screen
impl AddReleaseMainScreen {
    pub fn new(release: Release) -> Result<Self, Error> {
        let db = crate::database_with_polo::DatabaseWithPolo::get_instance();
        let games = db.get_all_games()?;
        let systems = db.get_systems()?;
        let settings = db.get_settings()?;
        let file_path_builder = FilePathBuilder::new(settings.collection_root_dir.clone());
        let files = db.get_collection_files(&release.files)?;

        Ok(Self {
            games,
            selected_game: None,
            release,
            systems,
            selected_file: HashMap::new(),
            selected_file_type: None,
            settings,
            file_path_builder,
            files,
        })
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ManageGames => Action::ManageGames,
            Message::ManageSystems => Action::ManageSystems,
            Message::Back => Action::Back,
            Message::GameSelected(game) => Action::GameSelected(game),
            Message::NameChanged(name) => Action::NameChanged(name),
            Message::SystemSelected(system) => Action::SystemSelected(system),
            Message::SelectFile => Action::Run(Task::perform(pick_file(), Message::FilePicked)),
            Message::FilePicked(result) => {
                let selected_system = self.systems.iter().find(|system| {
                    self.release
                        .system_id
                        .map_or(false, |system_id| system.id() == system_id)
                });

                if let (Some(system), Some(selected_file_type)) =
                    (selected_system, self.selected_file_type.clone())
                {
                    match result {
                        Ok(picked_file) => {
                            let collection_file = CollectionFile {
                                _id: None,
                                original_file_name: picked_file.file_name.clone(),
                                collection_file_type: self.selected_file_type.clone().unwrap(),
                                files: picked_file.files.clone(),
                                is_zip: picked_file.is_zip,
                            };
                            let db = DatabaseWithPolo::get_instance();
                            match db.add_collection_file(&collection_file) {
                                Ok(id) => Action::Run(Task::perform(
                                    copy_file(
                                        self.file_path_builder
                                            .build_target_directory(system, &selected_file_type),
                                        id,
                                        picked_file,
                                    ),
                                    Message::FileCopied,
                                )),
                                Err(err) => Action::Error(err),
                            }
                        }
                        Err(_) => Action::None,
                    }
                } else {
                    Action::None
                }
            }
            Message::FileCopied(result) => match result {
                Ok(id) => Action::AddFile(id),
                // TODO: if copy fails, remove the file from the database
                Err(err) => Action::Error(err),
            },
            Message::Submit => Action::Submit,
            Message::Clear => Action::Clear,
            Message::FileSelected(id, file) => {
                self.selected_file.insert(id, file);
                Action::None
            }
            Message::CollectionFileTypeSelected(file_type) => {
                self.selected_file_type = Some(file_type);
                Action::None
            }
            Message::ViewImage(file_path) => Action::ViewImage(file_path),
            Message::DeleteFile(id) => {
                if let Some(system) = self.get_release_system() {
                    if let Some(file) = self.files.iter().find(|f| f.id() == id) {
                        if let Ok(file_path) = self.file_path_builder.build_file_path(system, file)
                        {
                            return Action::Run(Task::perform(
                                delete_file(file_path.clone()),
                                move |result| Message::FileDeleted(result, id.clone()),
                            ));
                        }
                    }
                }
                Action::None
            }
            Message::FileDeleted(result, id) => match result {
                Ok(_) => {
                    if let Some(file_id) = self.release.files.iter().find(|f| **f == id) {
                        return Action::DeleteFile(file_id.clone());
                    }
                    Action::None
                }
                Err(err) => Action::Error(err),
            },
            Message::Save => Action::Save,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let back_button = button("Back").on_press(Message::Back);
        let release_name_input_field =
            text_input("Enter release name", &self.release.name).on_input(Message::NameChanged);
        let selected_games_list = self.create_selected_games_list();
        let manage_games_button: button::Button<'_, Message> = button("Manage Games")
            .width(iced::Length::Fixed(200.0))
            .on_press(Message::ManageGames);

        let selected_system = self.get_release_system();

        let systems_select = pick_list(
            self.systems.as_slice(),
            selected_system,
            Message::SystemSelected,
        );
        let manage_systems_button = button("Manage Systems")
            .width(iced::Length::Fixed(200.0))
            .on_press(Message::ManageSystems);

        let file_picker_row = self.create_file_picker();
        let emulator_files_list = self.create_emulator_files_list();
        let scan_files_list = self.create_scan_files_list();

        let main_buttons = row![
            button("Save").on_press(Message::Save),
            button("Submit").on_press(Message::Submit),
            button("Clear").on_press(Message::Clear)
        ];

        column![
            back_button,
            release_name_input_field,
            selected_games_list,
            manage_games_button,
            systems_select,
            manage_systems_button,
            file_picker_row,
            emulator_files_list,
            scan_files_list,
            main_buttons
        ]
        .into()
    }

    fn get_release_system(&self) -> Option<&System> {
        self.systems.iter().find(|system| {
            self.release
                .system_id
                .map_or(false, |system_id| system.id() == system_id)
        })
    }

    fn create_selected_games_list(&self) -> Element<Message> {
        let selected_games_title = text("Selected Games:");

        let selected_games_list = self
            .release
            .games
            .iter()
            .map(|game_id| {
                let game = self
                    .games
                    .iter()
                    .find(|game| game.id() == *game_id)
                    .unwrap();
                text(&game.name).into()
            })
            .collect::<Vec<Element<Message>>>();

        let available_games: Vec<Game> = self
            .games
            .iter()
            .filter(|g| !self.release.games.contains(&g.id()))
            .cloned()
            .collect();

        let game_picker = pick_list(
            available_games,
            self.selected_game.clone(),
            Message::GameSelected,
        );

        column![
            selected_games_title,
            Column::with_children(selected_games_list),
            game_picker
        ]
        .into()
    }

    fn create_file_picker(&self) -> Element<Message> {
        let collection_file_type_picker = pick_list(
            vec![
                CollectionFileType::Rom,
                CollectionFileType::DiskImage,
                CollectionFileType::CoverScan,
                CollectionFileType::Manual,
                CollectionFileType::ScreenShot,
            ],
            self.selected_file_type.clone(),
            Message::CollectionFileTypeSelected,
        );
        let add_file_button = button("Add File").on_press_maybe(
            (self.release.system_id.is_some() && self.selected_file_type.is_some())
                .then(|| Message::SelectFile),
        );
        row![collection_file_type_picker, add_file_button].into()
    }

    fn create_scan_files_list(&self) -> Element<Message> {
        let scan_files_list = self
            .files
            .iter()
            .filter(|f| f.collection_file_type == CollectionFileType::CoverScan)
            .filter_map(|file| {
                if let Some(system) = self.get_release_system() {
                    if let Ok(thumb_path) = get_thumbnail_path(file, &self.settings, system) {
                        if let Ok(file_path) = self.file_path_builder.build_file_path(system, file)
                        {
                            let image = image(thumb_path);
                            let view_image_button =
                                button(image).on_press(Message::ViewImage(file_path));
                            let delete_button =
                                button("Delete").on_press(Message::DeleteFile(file.id()));
                            return Some(row![view_image_button, delete_button].into());
                        }
                    }
                }

                None
            })
            .collect::<Vec<iced::Element<Message>>>();
        Column::with_children(scan_files_list).into()
    }

    fn create_emulator_files_list(&self) -> Element<Message> {
        let files_list = self
            .files
            .iter()
            .filter(|f| {
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
                    move |selected_file_name| Message::FileSelected(file.id(), selected_file_name),
                );
                row![container_filename, file_picker,].into()
            })
            .collect::<Vec<iced::Element<Message>>>();
        Column::with_children(files_list).into()
    }
}
