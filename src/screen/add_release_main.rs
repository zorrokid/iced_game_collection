use crate::database::Database;
use crate::emulator_runner::EmulatorRunOptions;
use crate::manage_games;
use crate::manage_systems;
use crate::model::model::Release;
use crate::screen::add_release_screen::add_release_main_screen;
use crate::screen::add_release_screen::AddReleaseScreen;
use iced::{Element, Task};
use uuid::Uuid;

use super::view_image;

#[derive(Debug, Clone)]
pub struct AddReleaseMain {
    screen: AddReleaseScreen,
    // release to be added or edited, sub screens will submit events to update this
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
    RunWithEmulator(EmulatorRunOptions),
    Error(String),
}

impl AddReleaseMain {
    pub fn new(edit_release_id: Option<String>) -> Self {
        let db = Database::get_instance();
        /* let releases = db.read().unwrap().to_release_list_model();*/
        let edit_release = edit_release_id.and_then(|id| db.read().unwrap().get_release(&id));
        let release = match edit_release {
            Some(release) => release,
            None => Release::default(),
        };
        Self {
            screen: create_main_screen(&release),
            release,
        }
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
                            self.screen = AddReleaseScreen::ManageGamesScreen(
                                manage_games::ManageGames::new(None),
                            );
                            Action::None
                        }
                        add_release_main_screen::Action::ManageSystems => {
                            self.handle_navigate_to_manage_systems(None)
                        }
                        add_release_main_screen::Action::Back => Action::Back,
                        add_release_main_screen::Action::GameSelected(game) => {
                            self.release.games.push(game.id);
                            self.screen = create_main_screen(&self.release);
                            Action::None
                        }
                        add_release_main_screen::Action::None => Action::None,
                        add_release_main_screen::Action::NameChanged(name) => {
                            self.release.name = name;
                            self.screen = create_main_screen(&self.release);
                            Action::None
                        }
                        add_release_main_screen::Action::SystemSelected(system) => {
                            self.release.system_id = system.id;
                            self.screen = create_main_screen(&self.release);
                            Action::None
                        }
                        add_release_main_screen::Action::AddFile(file) => {
                            self.release.files.push(file);
                            self.screen = create_main_screen(&self.release);
                            Action::None
                        }
                        add_release_main_screen::Action::Run(task) => {
                            Action::Run(task.map(Message::AddReleaseMainScreen))
                        }
                        add_release_main_screen::Action::Submit(/*release*/) => {
                            self.update_release();
                            Action::ReleaseSubmitted
                        }
                        add_release_main_screen::Action::Clear => {
                            //let db = Database::get_instance();
                            //let releases = db.read().unwrap().to_release_list_model();
                            self.release = Release::default();
                            self.screen = create_main_screen(&self.release);
                            Action::None
                        }
                        add_release_main_screen::Action::RunWithEmulator(options) => {
                            Action::RunWithEmulator(options)
                        }
                        add_release_main_screen::Action::ViewImage(file) => {
                            self.screen =
                                AddReleaseScreen::ViewImageScreen(view_image::ViewImage::new(file));
                            Action::None
                        }
                        add_release_main_screen::Action::Error(error) => Action::Error(error),
                        add_release_main_screen::Action::DeleteFile(file) => {
                            self.release.files.retain(|f| f.id != file.id);
                            self.update_release();
                            self.screen = create_main_screen(&self.release);
                           Action::None
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
                            self.screen = create_main_screen(&self.release);
                            Action::None
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
                        manage_systems::Action::GoHome => {
                            self.screen = create_main_screen(&self.release);
                            Action::None
                        }
                        manage_systems::Action::SystemDeleted => {
                            self.handle_navigate_to_manage_systems(None)
                        }
                        manage_systems::Action::EditSystem(id) => {
                            self.handle_navigate_to_manage_systems(Some(id))
                        }
                        manage_systems::Action::None => Action::None,
                        manage_systems::Action::SystemSubmitted => {
                            self.screen = create_main_screen(&self.release);
                            Action::None
                        }
                        manage_systems::Action::Error(error) => Action::Error(error),
                    }
                } else {
                    Action::None
                }
            }
            Message::ViewImageScreen(sub_screen_message) => {
                if let AddReleaseScreen::ViewImageScreen(sub_screen) = &mut self.screen {
                    match sub_screen.update(sub_screen_message) {
                        view_image::Action::Back => {
                            self.screen = create_main_screen(&self.release);
                            Action::None
                        }
                        _ => Action::None,
                    }
                } else {
                    Action::None
                }
            }
        }
    }

    fn handle_navigate_to_manage_systems(&mut self, id: Option<String>) -> Action {
        match manage_systems::ManageSystems::new(id) {
            Ok(screen) => {
                self.screen = AddReleaseScreen::ManageSystemsScreen(screen);
                Action::None
            }
            Err(e) => Action::Error(e.to_string()),
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
    fn update_release(&mut self) {
        let db = Database::get_instance();
        db.write()
            .unwrap()
            .add_or_update_release(self.release.clone());
    }
}

fn create_main_screen(release: &Release) -> AddReleaseScreen {
    AddReleaseScreen::AddReleaseMainScreen(add_release_main_screen::AddReleaseMainScreen::new(
        release.clone(),
    ))
}
