use crate::database::Database;
use crate::manage_games;
use crate::manage_systems;
use crate::model::init_new_release;
use crate::model::Release;
use crate::screen::add_release_screen::add_release_main_screen;
use crate::screen::add_release_screen::AddReleaseScreen;
use iced::{Element, Task};

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
}

pub enum Action {
    Back,
    None,
    Run(Task<Message>),
    Error(String),
    ReleaseSubmitted,
}

impl AddReleaseMain {
    pub fn new(edit_release_id: Option<i32>) -> Self {
        let db = Database::get_instance();
        let releases = db.read().unwrap().to_release_list_model();
        let edit_release = edit_release_id.and_then(|id| db.read().unwrap().get_release(id));
        let release = match edit_release {
            Some(release) => release,
            None => init_new_release(&releases),
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
                            self.screen = AddReleaseScreen::ManageSystemsScreen(
                                manage_systems::ManageSystems::new(None),
                            );
                            Action::None
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
                        add_release_main_screen::Action::Submit(release) => {
                            let db = Database::get_instance();
                            db.write().unwrap().add_or_update_release(release);
                            Action::ReleaseSubmitted
                        }
                        add_release_main_screen::Action::Clear => {
                            let db = Database::get_instance();
                            let releases = db.read().unwrap().to_release_list_model();
                            self.release = init_new_release(&releases);
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
                            self.screen = AddReleaseScreen::ManageSystemsScreen(
                                manage_systems::ManageSystems::new(None),
                            );
                            Action::None
                        }
                        manage_systems::Action::EditSystem(id) => {
                            self.screen = AddReleaseScreen::ManageSystemsScreen(
                                manage_systems::ManageSystems::new(Some(id)),
                            );
                            Action::None
                        }
                        manage_systems::Action::None => Action::None,
                        manage_systems::Action::Run(task) => {
                            Action::Run(task.map(Message::ManageSystemsScreen))
                        }
                        manage_systems::Action::SystemSubmitted => {
                            self.screen = create_main_screen(&self.release);
                            Action::None
                        }
                    }
                } else {
                    Action::None
                }
            }
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
        }
    }
}

fn create_main_screen(release: &Release) -> AddReleaseScreen {
    AddReleaseScreen::AddReleaseMainScreen(add_release_main_screen::AddReleaseMainScreen::new(
        release.clone(),
    ))
}
