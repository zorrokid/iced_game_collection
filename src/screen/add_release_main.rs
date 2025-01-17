use crate::database_with_polo::DatabaseWithPolo;
use crate::error::Error;
use crate::manage_games;
use crate::manage_systems;
use crate::model::model::HasOid;
use crate::model::model::Release;
use crate::repository::repository::ReleaseReadRepository;
use crate::screen::add_release_screen::add_release_main_screen;
use crate::screen::add_release_screen::AddReleaseScreen;
use bson::oid::ObjectId;
use iced::{Element, Task};

use super::view_image;

#[derive(Debug, Clone)]
pub struct AddReleaseMain {
    screen: AddReleaseScreen,
    // release to be added or edited, sub screens will submit events to update this
    // NOTE! Do not move state to sub screen, when moving between screens, changes won't be lost event without saving to db
    // Also, we don't want to save to db after each state, because of cancel functionality.
    // Only changes that are saved to db immediately are adding or deleting files because actual files are copied or deleted.
    release: Release,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddReleaseMainScreen(add_release_main_screen::Message),
    ManageGamesScreen(manage_games::Message),
    ManageSystemsScreen(manage_systems::Message),
    ViewImageScreen(view_image::Message),
}

pub enum Action {
    Back,
    None,
    Run(Task<Message>),
    ReleaseSubmitted,
    Error(Error),
}

impl AddReleaseMain {
    pub fn new(edit_release_id: Option<ObjectId>) -> Result<Self, Error> {
        let db = DatabaseWithPolo::get_instance();

        let edit_release = match edit_release_id {
            Some(id) => db.get_release(&id)?,
            None => None,
        };

        let release = match edit_release {
            Some(release) => release,
            None => Release::default(),
        };
        let screen = add_release_main_screen::AddReleaseMainScreen::new(release.clone())?;

        Ok(Self {
            screen: AddReleaseScreen::AddReleaseMainScreen(screen),
            release,
        })
    }

    pub fn title(&self) -> String {
        "Add Release".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::AddReleaseMainScreen(sub_screen_message) => {
                if let AddReleaseScreen::AddReleaseMainScreen(sub_screen) = &mut self.screen {
                    match sub_screen.update(sub_screen_message) {
                        add_release_main_screen::Action::ManageGames => {
                            match manage_games::ManageGames::new(None) {
                                Ok(screen) => {
                                    self.screen = AddReleaseScreen::ManageGamesScreen(screen);
                                    Action::None
                                }
                                Err(e) => Action::Error(e),
                            }
                        }
                        add_release_main_screen::Action::ManageSystems => {
                            self.handle_navigate_to_manage_systems(None)
                        }
                        add_release_main_screen::Action::Back => Action::Back,
                        add_release_main_screen::Action::GameSelected(game) => {
                            self.release.games.push(game.id());
                            self.switch_main_screen()
                        }
                        add_release_main_screen::Action::None => Action::None,
                        add_release_main_screen::Action::NameChanged(name) => {
                            self.release.name = name;
                            self.switch_main_screen()
                        }
                        add_release_main_screen::Action::SystemSelected(system) => {
                            self.release.system_id = Some(system.id());
                            self.switch_main_screen()
                        }
                        add_release_main_screen::Action::AddFile(file) => {
                            self.release.files.push(file);

                            match self.update_release() {
                                Ok(_) => self.switch_main_screen(),
                                Err(e) => Action::Error(e),
                            }
                        }
                        add_release_main_screen::Action::Run(task) => {
                            Action::Run(task.map(Message::AddReleaseMainScreen))
                        }
                        add_release_main_screen::Action::Submit => match self.update_release() {
                            Ok(_) => Action::ReleaseSubmitted,
                            Err(e) => Action::Error(e),
                        },
                        add_release_main_screen::Action::Save => match self.update_release() {
                            Ok(_) => self.switch_main_screen(),
                            Err(e) => Action::Error(e),
                        },
                        add_release_main_screen::Action::Clear => {
                            self.release = Release::default();
                            self.switch_main_screen()
                        }
                        add_release_main_screen::Action::ViewImage(file) => {
                            self.screen =
                                AddReleaseScreen::ViewImageScreen(view_image::ViewImage::new(file));
                            Action::None
                        }
                        add_release_main_screen::Action::Error(error) => Action::Error(error),
                        add_release_main_screen::Action::DeleteFile(file_id) => {
                            self.release.files.retain(|f| *f != file_id);
                            match self.update_release() {
                                Ok(_) => self.switch_main_screen(),
                                Err(e) => Action::Error(e),
                            }
                        }
                    }
                } else {
                    Action::None
                }
            }
            Message::ManageGamesScreen(sub_screen_message) => {
                if let AddReleaseScreen::ManageGamesScreen(sub_screen) = &mut self.screen {
                    match sub_screen.update(sub_screen_message) {
                        manage_games::Action::GameSubmitted | manage_games::Action::Back => {
                            self.switch_main_screen()
                        }
                        _ => Action::None,
                    }
                } else {
                    Action::None
                }
            }
            Message::ManageSystemsScreen(sub_screen_message) => {
                if let AddReleaseScreen::ManageSystemsScreen(sub_screen) = &mut self.screen {
                    match sub_screen.update(sub_screen_message) {
                        manage_systems::Action::GoHome => self.switch_main_screen(),
                        manage_systems::Action::EditSystem(id) => {
                            self.handle_navigate_to_manage_systems(Some(id))
                        }
                        manage_systems::Action::None => Action::None,
                        manage_systems::Action::SystemSubmitted => self.switch_main_screen(),
                        manage_systems::Action::Error(error) => Action::Error(error),
                    }
                } else {
                    Action::None
                }
            }
            Message::ViewImageScreen(sub_screen_message) => {
                if let AddReleaseScreen::ViewImageScreen(sub_screen) = &mut self.screen {
                    match sub_screen.update(sub_screen_message) {
                        view_image::Action::Back => self.switch_main_screen(),
                        _ => Action::None,
                    }
                } else {
                    Action::None
                }
            }
        }
    }

    fn handle_navigate_to_manage_systems(&mut self, id: Option<ObjectId>) -> Action {
        match manage_systems::ManageSystems::new(id) {
            Ok(screen) => {
                self.screen = AddReleaseScreen::ManageSystemsScreen(screen);
                Action::None
            }
            Err(e) => Action::Error(e),
        }
    }

    pub fn view(&self) -> Element<Message> {
        match &self.screen {
            AddReleaseScreen::AddReleaseMainScreen(screen) => {
                screen.view().map(Message::AddReleaseMainScreen)
            }
            AddReleaseScreen::ManageGamesScreen(screen) => {
                screen.view().map(Message::ManageGamesScreen)
            }
            AddReleaseScreen::ManageSystemsScreen(screen) => {
                screen.view().map(Message::ManageSystemsScreen)
            }
            AddReleaseScreen::ViewImageScreen(screen) => {
                screen.view().map(Message::ViewImageScreen)
            }
        }
    }
    fn update_release(&mut self) -> Result<ObjectId, Error> {
        let db = DatabaseWithPolo::get_instance();
        match self.release._id.is_some() {
            true => db.update_release(&self.release),
            false => {
                let id = db.add_release(&self.release)?;
                if let Some(release) = db.get_release(&id)? {
                    self.release = release;
                }
                Ok(id)
            }
        }
    }

    fn switch_main_screen(&mut self) -> Action {
        let screen = add_release_main_screen::AddReleaseMainScreen::new(self.release.clone());
        match screen {
            Ok(screen) => {
                self.screen = AddReleaseScreen::AddReleaseMainScreen(screen);
                Action::None
            }
            Err(err) => Action::Error(err),
        }
    }
}
