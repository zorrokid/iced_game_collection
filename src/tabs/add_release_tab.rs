use std::{collections::HashMap, path::PathBuf};

use bson::oid::ObjectId;
use iced::{
    widget::{button, column, container, image, pick_list, row, text, text_input, Column},
    Element, Task,
};

use crate::{
    database_with_polo::DatabaseWithPolo,
    error::Error,
    model::{
        collection_file::{CollectionFile, CollectionFileType},
        model::{Game, HasOid, Release, Settings, System},
    },
    repository::repository::{
        CollectionFilesReadRepository as _, ReleaseReadRepository, SystemReadRepository,
    },
    util::{file_path_builder::FilePathBuilder, image::get_thumbnail_path},
};

use super::widgets::{add_game_widget, add_system_widget, file_select_widget};

pub struct AddReleaseTab {
    release: Release,
    games: Vec<Game>,
    selected_game: Option<Game>,
    selected_games: Vec<ObjectId>,
    add_game_widget: add_game_widget::AddGame,
    add_system_widget: add_system_widget::AddSystem,
    file_select_widget: file_select_widget::FileSelect,
    adding_game: bool,
    adding_system: bool,
    //release_name: String,
    systems: Vec<System>,
    selected_system: Option<System>,
    files: Vec<CollectionFile>,
    settings: Settings,
    file_path_builder: FilePathBuilder,
    can_cancel: bool,
    selected_file: HashMap<ObjectId, String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddGame(add_game_widget::Message),
    AddSystem(add_system_widget::Message),
    FileSelect(file_select_widget::Message),
    GameSelected(Game),
    RemoveGame(ObjectId),
    StartAddingGame,
    StartAddingSystem,
    ReleaseNameUpdated(String),
    SystemSelected(System),
    ViewImage(PathBuf),
    DeleteFile(ObjectId),
    FileDeleted(Result<(), Error>, ObjectId),
    FileSelected(ObjectId, String),
}

impl AddReleaseTab {
    pub fn new(release_id: Option<ObjectId>) -> Self {
        let db = DatabaseWithPolo::get_instance();
        let games = db.get_all_games().unwrap_or_else(|err| {
            println!("Failed to get games {:?}", err);
            vec![]
        });
        let systems = db.get_all_systems().unwrap_or_else(|err| {
            println!("Failed to get systems {:?}", err);
            vec![]
        });
        let settings = db.get_settings().unwrap_or_default();
        let file_path_builder = FilePathBuilder::new(settings.collection_root_dir.clone());

        let release = match release_id {
            Some(id) => match db.get_release(&id) {
                Ok(Some(release)) => release,
                Ok(None) => {
                    eprintln!("No release found with id {}", id);
                    Release::default()
                }
                Err(err) => {
                    eprintln!("Failed to get release with id {}: {}", id, err);
                    Release::default()
                }
            },
            None => Release::default(),
        };

        let files = match db.get_collection_files(&release.files) {
            Ok(files) => files,
            Err(err) => {
                eprintln!("Failed to get collection files: {}", err);
                vec![]
            }
        };

        Self {
            add_game_widget: add_game_widget::AddGame::new(),
            add_system_widget: add_system_widget::AddSystem::new(),
            file_select_widget: file_select_widget::FileSelect::new(
                settings.collection_root_dir.clone(),
            ),
            games,
            adding_game: false,
            adding_system: false,
            selected_game: None,
            selected_games: vec![],
            systems,
            selected_system: None,
            files,
            settings,
            file_path_builder,
            release,
            can_cancel: true,
            selected_file: HashMap::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AddGame(message) => {
                match self.add_game_widget.update(message) {
                    add_game_widget::Action::GameAdded(game) => {
                        self.games.push(game);
                        self.adding_game = false;
                    }
                    add_game_widget::Action::CancelAddGame => {
                        self.adding_game = false;
                    }
                    add_game_widget::Action::None => {}
                }
                Task::none()
            }
            Message::AddSystem(message) => {
                match self.add_system_widget.update(message) {
                    add_system_widget::Action::SystemAdded(system) => {
                        self.systems.push(system);
                        self.adding_system = false;
                    }
                    add_system_widget::Action::CancelAddSystem => {
                        self.adding_system = false;
                    }
                    add_system_widget::Action::None => {}
                }
                Task::none()
            }
            Message::FileSelect(message) => {
                match self.file_select_widget.update(message) {
                    file_select_widget::Action::None => {
                        // do nothing
                    }
                    file_select_widget::Action::AddFile(file_id) => {
                        self.release.files.push(file_id);
                    }
                    file_select_widget::Action::Run(task) => {
                        return task.map(Message::FileSelect);
                    }
                }
                Task::none()
            }
            Message::StartAddingGame => {
                self.adding_game = true;
                Task::none()
            }
            Message::StartAddingSystem => {
                self.adding_system = true;
                Task::none()
            }
            Message::GameSelected(game) => {
                let game_id = game.id();
                self.selected_game = Some(game);
                self.selected_games.push(game_id);
                Task::none()
            }
            Message::RemoveGame(remove_id) => {
                self.selected_games.retain(|id| *id != remove_id);
                Task::none()
            }
            Message::ReleaseNameUpdated(name) => {
                self.release.name = name;
                Task::none()
            }
            Message::SystemSelected(system) => {
                let system_id = system.id();
                self.release.system_id = Some(system.id());
                self.file_select_widget
                    .update(file_select_widget::Message::SetSystemId(system_id));
                Task::none()
            }
            Message::DeleteFile(id) => {
                if let Some(file_to_be_deleted) = self.files.iter().find(|f| f.id() == id).cloned()
                {
                    // TODO do we need to maintain both files (CollectionFile) and release.files
                    // (ObjectId)?
                    self.files.retain(|f| f.id() != id);
                    self.release.files.retain(|f| *f != id);

                    let db = DatabaseWithPolo::get_instance();
                    match db.update_release(&self.release) {
                        Ok(_) => {
                            self.can_cancel = false;
                        }
                        Err(err) => {
                            eprintln!("Failed to update release: {}", err);
                        }
                    }
                    self.file_select_widget
                        .update(file_select_widget::Message::DeleteFile(file_to_be_deleted));
                }
                Task::none()
            }
            Message::ViewImage(_file_path) => {
                // TODO
                Task::none()
            }
            Message::FileSelected(id, file) => {
                self.selected_file.insert(id, file);
                Task::none()
            }
            Message::FileDeleted(_result, _id) => {
                // TODO
                Task::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let release_name_input =
            text_input("Release Name", &self.release.name).on_input(Message::ReleaseNameUpdated);
        let games_row = row![
            self.create_games_dropdown(),
            button("Add Game")
                .on_press_maybe((!self.adding_game).then_some(Message::StartAddingGame)),
        ];

        let selected_games_list = self.create_selected_games_list();

        let systems_row = row![
            self.create_systems_dropdown(),
            button("Add System")
                .on_press_maybe((!self.adding_system).then_some(Message::StartAddingSystem)),
        ];

        let add_game_row = if self.adding_game {
            self.create_add_game_container()
        } else {
            row![].into()
        };

        let add_system_row = if self.adding_system {
            self.create_add_system_container()
        } else {
            row![].into()
        };

        let file_select = self.file_select_widget.view().map(Message::FileSelect);
        let emulator_files_list = self.create_emulator_files_list();
        let scan_files_list = self.create_files_list(CollectionFileType::CoverScan);
        let screenshot_files_list = self.create_files_list(CollectionFileType::Screenshot);

        column![
            release_name_input,
            games_row,
            add_game_row,
            selected_games_list,
            systems_row,
            add_system_row,
            file_select,
            emulator_files_list,
            scan_files_list,
            screenshot_files_list,
            button("Submit"),
        ]
        .into()
    }

    fn create_add_game_container(&self) -> iced::Element<Message> {
        let content = self.add_game_widget.view().map(Message::AddGame);
        container(content)
            .padding(10)
            .style(container::rounded_box)
            .into()
    }

    fn create_add_system_container(&self) -> iced::Element<Message> {
        let content = self.add_system_widget.view().map(Message::AddSystem);
        container(content)
            .padding(10)
            .style(container::rounded_box)
            .into()
    }

    fn create_games_dropdown(&self) -> iced::Element<Message> {
        pick_list(
            self.games.clone(),
            self.selected_game.clone(),
            Message::GameSelected,
        )
        .into()
    }

    fn create_systems_dropdown(&self) -> iced::Element<Message> {
        pick_list(
            self.systems.clone(),
            self.selected_system.clone(),
            Message::SystemSelected,
        )
        .into()
    }

    // TODO: selected games list could be a separate widget
    fn create_selected_games_list(&self) -> iced::Element<Message> {
        let selected_games_title = text("Games in release:");

        let selected_games_list = self
            .games
            .iter()
            .filter(|game| self.selected_games.contains(&game.id()))
            .map(|game| {
                row![
                    text(&game.name),
                    button("Remove").on_press(Message::RemoveGame(game.id()))
                ]
                .into()
            })
            .collect::<Vec<iced::Element<Message>>>();

        column![
            selected_games_title,
            Column::with_children(selected_games_list)
        ]
        .into()
    }

    fn create_files_list(&self, file_type: CollectionFileType) -> Element<Message> {
        let files_list = self
            .files
            .iter()
            .filter(|f| f.collection_file_type == file_type)
            .filter_map(|file| {
                if let Some(system_id) = self.release.system_id {
                    if let Ok(thumb_path) =
                        get_thumbnail_path(file, &self.settings.collection_root_dir, &system_id)
                    {
                        if let Ok(file_path) =
                            self.file_path_builder.build_file_path(&system_id, file)
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
