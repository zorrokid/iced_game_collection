use std::path::PathBuf;
use std::sync::Arc;
use std::{collections::HashMap, vec};

use crate::database::collection_file_repository::CollectionFileReadRepository;
use crate::database::database_error::DatabaseError;
use crate::database::repository_manager::RepositoryManager;
use crate::database::software_title_repository::SoftwareTitleReadRepository as _;
use crate::database::system_repository::SystemReadRepository;
use crate::error::Error;
use crate::files::{copy_file, delete_file, pick_file, PickedFile};
use crate::model::model::HasOid;
use crate::model::{
    collection_file::{CollectionFile, CollectionFileType},
    model::{Release, SoftwareTitle, System},
};
use crate::util::file_path_builder::FilePathBuilder;
use crate::util::image::get_thumbnail_path;
use crate::view_model::settings::Settings;
use iced::widget::{button, column, image, pick_list, row, text, text_input, Column};
use iced::{Element, Task};

#[derive(Debug, Clone)]
pub struct AddReleaseMainScreen {
    software_titles: Vec<SoftwareTitle>,
    selected_software_title: Option<SoftwareTitle>,
    release: Release,
    systems: Vec<System>,
    selected_file: HashMap<i64, String>,
    selected_file_type: Option<CollectionFileType>,
    settings: Arc<Settings>,
    files: Vec<CollectionFile>,
    repo: Arc<RepositoryManager>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ManageGames,
    ManageSystems,
    Back,
    GameSelected(SoftwareTitle),
    NameChanged(String),
    SystemSelected(System),
    SelectFile,
    FilePicked(Result<PickedFile, Error>),
    Submit,
    Clear,
    FileSelected(i64, String),
    CollectionFileTypeSelected(CollectionFileType),
    ViewImage(PathBuf),
    FileCopied(Result<i64, Error>),
    DeleteFile(i64),
    FileDeleted(Result<(), Error>, i64),
    Save,
    SoftwareTitlesLoaded(Result<Vec<SoftwareTitle>, DatabaseError>),
    SystemsLoaded(Result<Vec<System>, DatabaseError>),
    CollectionFilesLoaded(Result<Vec<CollectionFile>, DatabaseError>),
}

pub enum Action {
    ManageGames,
    ManageSystems,
    Back,
    GameSelected(SoftwareTitle),
    NameChanged(String),
    None,
    SystemSelected(System),
    Run(Task<Message>),
    AddFile(i64),
    Submit,
    Clear,
    ViewImage(PathBuf),
    Error(Error),
    DeleteFile(i64),
    Save,
}

// TODO: add ViewReleaseScreen just for viewing release and using the view model
// Cannot use view model here because when adding a release, it doesn't have id yet
// TODO: split files management to another sub screen
impl AddReleaseMainScreen {
    pub fn new(
        release: Release,
        repo: Arc<RepositoryManager>,
        settings: Arc<Settings>,
    ) -> (Self, Task<Message>) {
        let repo_clone = Arc::clone(&repo);
        let load_games_task = Task::perform(
            async move { repo_clone.software_titles().get_all_software_titles().await },
            Message::SoftwareTitlesLoaded,
        );

        let repo_clone = Arc::clone(&repo);
        let load_systems_task = Task::perform(
            async move { repo_clone.systems().get_systems().await },
            Message::SystemsLoaded,
        );

        let repo_clone = Arc::clone(&repo);
        let load_collection_files_task = Task::perform(
            async move {
                repo_clone
                    .collection_files()
                    .get_collection_files_for_release(release.id)
                    .await
            },
            Message::CollectionFilesLoaded,
        );

        let combined_task = Task::batch(vec![
            load_games_task,
            load_systems_task,
            load_collection_files_task,
        ]);

        (
            Self {
                software_titles: vec![],
                selected_software_title: None,
                release,
                systems: vec![],
                selected_file: HashMap::new(),
                selected_file_type: None,
                settings,
                files: vec![],
                repo,
            },
            combined_task,
        )
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::SoftwareTitlesLoaded(Ok(software_titles)) => {
                self.software_titles = software_titles;
                Action::None
            }
            Message::SystemsLoaded(Ok(systems)) => {
                self.systems = systems;
                Action::None
            }
            Message::CollectionFilesLoaded(Ok(files)) => {
                self.files = files;
                Action::None
            }
            Message::ManageGames => Action::ManageGames,
            Message::ManageSystems => Action::ManageSystems,
            Message::Back => Action::Back,
            Message::GameSelected(game) => Action::GameSelected(game),
            Message::NameChanged(name) => Action::NameChanged(name),
            Message::SystemSelected(system) => Action::SystemSelected(system),
            Message::SelectFile => Action::Run(Task::perform(pick_file(), Message::FilePicked)),
            Message::FilePicked(result) => {
                if let (Some(system_id), Some(selected_file_type)) =
                    (self.release.system_id, self.selected_file_type.clone())
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
                                        self.file_path_builder.build_target_directory(
                                            &system_id,
                                            &selected_file_type,
                                        ),
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
                    if let Some(file) = self.files.iter().find(|f| f.id == id) {
                        if let Ok(file_path) =
                            FilePathBuilder::new(self.settings.collection_root_dir)
                                .build_file_path(system.id, file)
                        {
                            // TODO: remove also thumbnail if exists
                            return Action::Run(Task::perform(
                                delete_file(file_path.clone()),
                                move |result| Message::FileDeleted(result, id),
                            ));
                        }
                    }
                }
                Action::None
            }
            Message::FileDeleted(result, id) => match result {
                Ok(_) => {
                    if let Some(file_id) = self.release.files.iter().find(|f| **f == id) {
                        // TODO: maybe instead of spawning a task, just remove the file from the release
                        // => move logic from main to here
                        return Action::DeleteFile(*file_id);
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
        let scan_files_list = self.create_files_list(CollectionFileType::CoverScan);
        let screenshot_files_list = self.create_files_list(CollectionFileType::Screenshot);

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
            screenshot_files_list,
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
                    .software_titles
                    .iter()
                    .find(|game| game.id() == *game_id)
                    .unwrap();
                text(&game.name).into()
            })
            .collect::<Vec<Element<Message>>>();

        let available_games: Vec<SoftwareTitle> = self
            .software_titles
            .iter()
            .filter(|g| !self.release.games.contains(&g.id()))
            .cloned()
            .collect();

        let game_picker = pick_list(
            available_games,
            self.selected_software_title.clone(),
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
                CollectionFileType::Screenshot,
                CollectionFileType::TapeImage,
            ],
            self.selected_file_type.clone(),
            Message::CollectionFileTypeSelected,
        );
        let add_file_button = button("Add File").on_press_maybe(
            (self.release.system_id.is_some() && self.selected_file_type.is_some())
                .then_some(Message::SelectFile),
        );
        row![collection_file_type_picker, add_file_button].into()
    }

    fn create_files_list(&self, file_type: CollectionFileType) -> Element<Message> {
        let files_list = self
            .files
            .iter()
            .filter(|f| f.collection_file_type == file_type)
            .filter_map(|file| {
                if let Some(system) = self.get_release_system() {
                    if let Ok(thumb_path) =
                        get_thumbnail_path(file, &self.settings.collection_root_dir, &system.id())
                    {
                        if let Ok(file_path) =
                            self.file_path_builder.build_file_path(&system.id(), file)
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
        Column::with_children(files_list).into()
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
                let delete_button = button("Delete").on_press(Message::DeleteFile(file.id()));
                row![container_filename, file_picker, delete_button].into()
            })
            .collect::<Vec<iced::Element<Message>>>();
        Column::with_children(files_list).into()
    }
}
