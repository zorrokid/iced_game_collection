use crate::database::get_new_id;
use crate::manage_games;
use crate::manage_systems;
use crate::model::{Game, Release, ReleaseListModel, System};
use crate::screen::add_release_screen::add_release_main_screen;
use crate::screen::add_release_screen::AddReleaseScreen;
use iced::{Element, Task};

#[derive(Debug, Clone)]
pub struct AddReleaseMain {
    screen: AddReleaseScreen,
    games: Vec<Game>,
    // release to be added or edited, sub screens will submit events to update this
    release: Release,
    systems: Vec<System>,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddReleaseMainScreen(add_release_main_screen::Message),
    ManageGamesScreen(manage_games::Message),
    ManageSystemsScreen(manage_systems::Message),
}

pub enum Action {
    Back,
    SubmitRelease(Release),
    None,
    Run(Task<Message>),
    Error(String),
    SubmitGame(Game),
    DeleteSystem(i32),
    SubmitSystem(System),
}

impl AddReleaseMain {
    pub fn new(
        games: Vec<Game>,
        edit_release: Option<Release>,
        systems: Vec<System>,
        releases: Vec<ReleaseListModel>,
    ) -> Self {
        let release = match edit_release {
            Some(release) => release,
            None => Release {
                id: get_new_id(&releases),
                files: vec![],
                games: vec![],
                system_id: 0,
                name: "".to_string(),
            },
        };
        Self {
            screen: create_main_screen(&games, &release, &systems),
            games,
            release,
            systems,
        }
    }

    pub fn title(&self) -> String {
        "Add Release".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::AddReleaseMainScreen(sub_screen_message) => {
                if let AddReleaseScreen::AddReleaseMainScreen(sub_screen) = &mut self.screen {
                    let action = sub_screen.update(sub_screen_message);
                    match action {
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
                            self.screen =
                                create_main_screen(&self.games, &self.release, &self.systems);
                            Action::None
                        }
                        add_release_main_screen::Action::None => Action::None,
                        add_release_main_screen::Action::NameChanged(name) => {
                            self.release.name = name;
                            self.screen =
                                create_main_screen(&self.games, &self.release, &self.systems);
                            Action::None
                        }
                        add_release_main_screen::Action::SystemSelected(system) => {
                            self.release.system_id = system.id;
                            self.screen =
                                create_main_screen(&self.games, &self.release, &self.systems);
                            Action::None
                        }
                        add_release_main_screen::Action::AddFile(file) => {
                            self.release.files.push(file);
                            self.screen =
                                create_main_screen(&self.games, &self.release, &self.systems);
                            Action::None
                        }
                        add_release_main_screen::Action::Run(task) => {
                            Action::Run(task.map(Message::AddReleaseMainScreen))
                        }
                        add_release_main_screen::Action::Submit(release) => {
                            Action::SubmitRelease(release)
                        }
                    }
                } else {
                    Action::None
                }
            }
            Message::ManageGamesScreen(sub_screen_message) => {
                if let AddReleaseScreen::ManageGamesScreen(sub_screen) = &mut self.screen {
                    let action = sub_screen.update(sub_screen_message);
                    match action {
                        manage_games::Action::Back => {
                            self.screen =
                                create_main_screen(&self.games, &self.release, &self.systems);
                            Action::None
                        }
                        manage_games::Action::SubmitGame(game) => {
                            // TODO: would be better if local games wouldn't need to be updated explicitly
                            // but rather the list would reflect always what's in main (through a reference)?
                            self.games.push(game.clone());
                            self.screen =
                                create_main_screen(&self.games, &self.release, &self.systems);
                            Action::SubmitGame(game)
                        }
                        manage_games::Action::DeleteGame(id) => Action::None,
                        manage_games::Action::EditGame(id) => Action::None,
                        manage_games::Action::None => Action::None,
                    }
                } else {
                    Action::None
                }
            }
            Message::ManageSystemsScreen(sub_screen_message) => {
                if let AddReleaseScreen::ManageSystemsScreen(sub_screen) = &mut self.screen {
                    let action = sub_screen.update(sub_screen_message);
                    match action {
                        manage_systems::Action::GoHome => {
                            self.screen =
                                create_main_screen(&self.games, &self.release, &self.systems);
                            Action::None
                        }
                        manage_systems::Action::DeleteSystem(id) => Action::DeleteSystem(id),
                        manage_systems::Action::EditSystem(id) => {
                            let edit_system = self.systems.iter().find(|s| s.id == id).cloned();
                            self.screen = AddReleaseScreen::ManageSystemsScreen(
                                manage_systems::ManageSystems::new(edit_system),
                            );
                            Action::None
                        }
                        manage_systems::Action::None => Action::None,
                        manage_systems::Action::Run(task) => {
                            Action::Run(task.map(Message::ManageSystemsScreen))
                        }
                        manage_systems::Action::SubmitSystem(system) => {
                            // TODO: would be better if local systems wouldn't need to be updated explicitly
                            // but rather the list would reflect always what's in main (through a reference)?
                            self.systems.push(system.clone());
                            self.screen =
                                create_main_screen(&self.games, &self.release, &self.systems);
                            Action::SubmitSystem(system)
                        }
                    }
                } else {
                    Action::None
                }
            }
            _ => Action::None,
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

fn create_main_screen(
    games: &Vec<Game>,
    release: &Release,
    systems: &Vec<System>,
) -> AddReleaseScreen {
    AddReleaseScreen::AddReleaseMainScreen(add_release_main_screen::AddReleaseMainScreen::new(
        games.clone(),
        release.clone(),
        systems.clone(),
    ))
}
