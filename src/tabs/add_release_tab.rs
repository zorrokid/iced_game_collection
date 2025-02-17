use std::path::PathBuf;

use bson::oid::ObjectId;
use iced::{
    widget::{button, column, container, pick_list, row, text, text_input, Column},
    Task,
};

use crate::{
    database_with_polo::DatabaseWithPolo,
    error::Error,
    model::{
        collection_file::CollectionFile,
        model::{Game, HasOid, Release, Settings, System},
    },
    repository::repository::{
        CollectionFilesReadRepository as _, ReleaseReadRepository, SystemReadRepository,
    },
};

use super::widgets::{add_game_widget, add_system_widget, file_select_widget, files_list_widget};

pub struct AddReleaseTab {
    release: Release,
    games: Vec<Game>,
    selected_game: Option<Game>,
    add_game_widget: add_game_widget::AddGame,
    add_system_widget: add_system_widget::AddSystem,
    file_select_widget: file_select_widget::FileSelect,
    files_list: files_list_widget::FilesList,
    adding_game: bool,
    adding_system: bool,
    systems: Vec<System>,
    files: Vec<CollectionFile>,
    settings: Settings,
    is_saved: bool,
    can_save: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddGame(add_game_widget::Message),
    AddSystem(add_system_widget::Message),
    FileSelect(file_select_widget::Message),
    FilesList(files_list_widget::Message),
    GameSelected(Game),
    RemoveGame(ObjectId),
    StartAddingGame,
    StartAddingSystem,
    ReleaseNameUpdated(String),
    SystemSelected(System),
    ViewImage(PathBuf),
    //DeleteFile(ObjectId),
    //FileDeleted(Result<(), Error>, ObjectId),
    Cancel,
    Save,
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
            files_list: files_list_widget::FilesList::new(settings.collection_root_dir.clone()),
            games,
            adding_game: false,
            adding_system: false,
            selected_game: None,
            systems,
            files,
            settings,
            release,
            is_saved: false,
            can_save: false,
        }
    }

    fn set_can_save(&mut self) {
        self.can_save = !self.release.name.is_empty()
            && self.release.system_id.is_some()
            && !self.release.games.is_empty();
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
                        println!("Adding file to release: {:?}", file_id);
                        if let Some(system_id) = self.release.system_id {
                            self.release.files.push(file_id);
                            self.files_list.update(files_list_widget::Message::SetFiles(
                                self.release.files.clone(),
                                system_id,
                            ));
                        };
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
                self.release.games.push(game_id);
                Task::none()
            }
            Message::RemoveGame(remove_id) => {
                self.release.games.retain(|id| *id != remove_id);
                Task::none()
            }
            Message::ReleaseNameUpdated(name) => {
                self.release.name = name;
                self.set_can_save();
                Task::none()
            }
            Message::SystemSelected(system) => {
                let system_id = system.id();
                self.release.system_id = Some(system.id());
                self.file_select_widget
                    .update(file_select_widget::Message::SetSystemId(system_id));
                self.set_can_save();
                Task::none()
            }
            /*Message::DeleteFile(id) => {
                if let Some(file_to_be_deleted) = self.files.iter().find(|f| f.id() == id).cloned()
                {
                    // TODO do we need to maintain both files (CollectionFile) and release.files
                    // (ObjectId)?
                    self.files.retain(|f| f.id() != id);
                    self.release.files.retain(|f| *f != id);

                    let db = DatabaseWithPolo::get_instance();
                    match db.update_release(&self.release) {
                        Ok(_) => {
                            self.is_saved = true;
                        }
                        Err(err) => {
                            eprintln!("Failed to update release: {}", err);
                        }
                    }
                    self.files_list
                        .update(files_list_widget::Message::DeleteFile(file_to_be_deleted));
                }
                Task::none()
            }*/
            Message::ViewImage(_file_path) => {
                // TODO
                Task::none()
            }
            /*Message::FileDeleted(_result, _id) => {
                // TODO
                Task::none()
            }*/
            Message::Cancel => {
                if !self.is_saved {
                    self.release = Release::default();
                    self.selected_game = None;
                    self.adding_game = false;
                    self.adding_system = false;
                    self.files.clear();
                    self.file_select_widget
                        .update(file_select_widget::Message::Reset);
                }
                Task::none()
            }
            Message::Save => {
                self.save_release();
                Task::none()
            }
            Message::FilesList(message) => {
                match self.files_list.update(message) {
                    files_list_widget::Action::None => {}
                    files_list_widget::Action::RemoveFile(id) => {
                        if let Some(system_id) = self.release.system_id {
                            self.release.files.retain(|f| *f != id);
                            self.save_release();
                            self.files_list
                                .update(files_list_widget::Message::FileRemoved(id, system_id));
                        }
                    }
                    files_list_widget::Action::ViewImage(_file_path) => {
                        // TODO
                    }
                    files_list_widget::Action::Run(task) => {
                        return task.map(Message::FilesList);
                    }
                }

                Task::none()
            }
        }
    }

    fn save_release(&mut self) {
        let db = DatabaseWithPolo::get_instance();

        if self.release.has_id() {
            if let Err(err) = db.update_release(&self.release) {
                // TODO: show error to user
                eprintln!("Failed to update release: {}", err);
            } else {
                println!("Release updated");
                self.is_saved = true;
            }
        } else if let Err(err) = db.add_release(&self.release) {
            // TODO: show error to user
            eprintln!("Failed to add release: {}", err);
        } else {
            println!("Release added");
            self.is_saved = true;
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

        let file_select = if self.is_saved {
            self.file_select_widget.view().map(Message::FileSelect)
        } else {
            row![].into()
        };

        let cancel_button =
            button("Cancel").on_press_maybe((!self.is_saved).then_some(Message::Cancel));

        let save_button = button("Save").on_press_maybe(self.can_save.then_some(Message::Save));

        let files_list = self.files_list.view().map(Message::FilesList);
        column![
            release_name_input,
            games_row,
            add_game_row,
            selected_games_list,
            systems_row,
            add_system_row,
            file_select,
            files_list,
            row![cancel_button, save_button],
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
            self.systems
                .iter()
                .find(|s| self.release.system_id == Some(s.id()))
                .cloned(),
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
            .filter(|game| self.release.games.contains(&game.id()))
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
}
