mod database;
mod error;
mod files;
mod model;
mod screen;

use async_process::Command;
use async_std::path::Path;
use database::Database;
use error::Error;
use iced::{exit, Task};
use screen::add_release_main;
use screen::error as error_screen;
use screen::games;
use screen::home;
use screen::manage_emulators;
use screen::manage_games;
use screen::manage_systems;
use screen::view_game;

use crate::screen::Screen;

fn main() -> iced::Result {
    iced::application(
        IcedGameCollection::title,
        IcedGameCollection::update,
        IcedGameCollection::view,
    )
    .run_with(IcedGameCollection::new)
}

struct IcedGameCollection {
    screen: Screen,
}

#[derive(Debug, Clone)]
enum Message {
    Home(home::Message),
    Games(games::Message),
    ManageSystems(manage_systems::Message),
    ManageGames(manage_games::Message),
    ManageEmulators(manage_emulators::Message),
    AddReleaseMain(add_release_main::Message),
    ViewGame(screen::view_game::Message),
    FinishedRunningWithEmulator(Result<(), Error>),
    Error(error_screen::Message),
}

impl IcedGameCollection {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                screen: Screen::Home(home::Home::new()),
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        match &self.screen {
            Screen::Home(home) => home.title(),
            Screen::Games(games) => games.title(),
            Screen::ManageSystems(add_system) => add_system.title(),
            Screen::ManageGames(manage_games) => manage_games.title(),
            Screen::AddReleaseMain(add_release_main) => add_release_main.title(),
            Screen::ManageEmulators(add_emulator) => add_emulator.title(),
            Screen::ViewGame(view_game) => view_game.title(),
            Screen::Error(error) => error.title(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ManageSystems(message) => self.update_manage_systems(message),
            Message::ManageGames(message) => self.update_manage_games(message),
            Message::Home(message) => self.update_home(message),
            Message::Games(message) => self.update_games(message),
            Message::AddReleaseMain(message) => self.update_add_release(message),
            Message::ManageEmulators(message) => self.update_manage_emulators(message),
            Message::ViewGame(message) => self.update_view_game(message),
            Message::FinishedRunningWithEmulator(result) => {
                self.update_finished_running_emulator(result)
            }
            Message::Error(message) => self.update_error(message),
        }
    }

    fn view(&self) -> iced::Element<Message> {
        match &self.screen {
            Screen::Home(home) => home.view().map(Message::Home),
            Screen::Games(games) => games.view().map(Message::Games),
            Screen::ManageSystems(add_system) => add_system.view().map(Message::ManageSystems),
            Screen::ManageGames(manage_games) => manage_games.view().map(Message::ManageGames),
            Screen::AddReleaseMain(add_release_main) => {
                add_release_main.view().map(Message::AddReleaseMain)
            }
            Screen::ManageEmulators(add_emulator) => {
                add_emulator.view().map(Message::ManageEmulators)
            }
            Screen::ViewGame(view_game) => view_game.view().map(Message::ViewGame),
            Screen::Error(error) => error.view().map(Message::Error),
        }
    }

    fn update_manage_systems(&mut self, message: manage_systems::Message) -> Task<Message> {
        if let Screen::ManageSystems(add_system) = &mut self.screen {
            match add_system.update(message) {
                manage_systems::Action::SystemSubmitted | manage_systems::Action::SystemDeleted => {
                    self.screen = Screen::ManageSystems(screen::ManageSystems::new(None));
                    Task::none()
                }
                manage_systems::Action::None => Task::none(),
                manage_systems::Action::GoHome => {
                    self.screen = Screen::Home(screen::Home::new());
                    Task::none()
                }
                manage_systems::Action::EditSystem(id) => {
                    self.screen = Screen::ManageSystems(screen::ManageSystems::new(Some(id)));
                    Task::none()
                }
                manage_systems::Action::Run(task) => task.map(Message::ManageSystems),
            }
        } else {
            Task::none()
        }
    }

    fn update_manage_games(&mut self, message: manage_games::Message) -> Task<Message> {
        if let Screen::ManageGames(manage_games) = &mut self.screen {
            match manage_games.update(message) {
                manage_games::Action::Back => {
                    self.screen = Screen::Home(home::Home::new());
                    Task::none()
                }
                _ => Task::none(),
            }
        } else {
            Task::none()
        }
    }

    fn update_home(&mut self, message: home::Message) -> Task<Message> {
        if let Screen::Home(home) = &mut self.screen {
            match home.update(message) {
                home::Action::ViewGames => {
                    self.screen = Screen::Games(screen::Games::new());
                    Task::none()
                }
                home::Action::ManageSystems => {
                    self.screen = Screen::ManageSystems(screen::ManageSystems::new(None));
                    Task::none()
                }
                home::Action::ManageGames => {
                    self.screen = Screen::ManageGames(screen::ManageGames::new(None));
                    Task::none()
                }
                home::Action::AddRelease => {
                    self.screen = Screen::AddReleaseMain(screen::AddReleaseMain::new(None));
                    Task::none()
                }
                home::Action::Exit => match Database::get_instance().read().unwrap().save() {
                    Ok(_) => exit(),
                    Err(e) => {
                        eprintln!("Failed to save collection: {}", e);
                        Task::none()
                    }
                },
                home::Action::ManageEmulators => {
                    self.screen = Screen::ManageEmulators(screen::ManageEmulators::new(None));
                    Task::none()
                }
            }
        } else {
            Task::none()
        }
    }

    fn update_games(&mut self, message: games::Message) -> Task<Message> {
        if let Screen::Games(games) = &mut self.screen {
            match games.update(message) {
                games::Action::GoHome => {
                    self.screen = Screen::Home(home::Home::new());
                    Task::none()
                }
                games::Action::ViewGame(id) => {
                    self.screen = Screen::ViewGame(view_game::ViewGame::new(id));
                    Task::none()
                }
                _ => Task::none(),
            }
        } else {
            Task::none()
        }
    }

    fn update_add_release(&mut self, message: add_release_main::Message) -> Task<Message> {
        if let Screen::AddReleaseMain(add_release_main) = &mut self.screen {
            match add_release_main.update(message) {
                add_release_main::Action::Back | add_release_main::Action::ReleaseSubmitted => {
                    self.screen = Screen::Home(screen::Home::new());
                    Task::none()
                }
                add_release_main::Action::Run(task) => task.map(Message::AddReleaseMain),
                add_release_main::Action::None => Task::none(),
                add_release_main::Action::Error(e) => {
                    self.screen = Screen::Error(screen::Error::new(e));
                    Task::none()
                }
            }
        } else {
            Task::none()
        }
    }

    fn update_manage_emulators(&mut self, message: manage_emulators::Message) -> Task<Message> {
        if let Screen::ManageEmulators(add_emulator) = &mut self.screen {
            match add_emulator.update(message) {
                manage_emulators::Action::EmulatorSubmitted
                | manage_emulators::Action::EmulatorDeleted => {
                    self.screen = Screen::ManageEmulators(screen::ManageEmulators::new(None));
                    Task::none()
                }
                manage_emulators::Action::None => Task::none(),
                manage_emulators::Action::GoHome => {
                    self.screen = Screen::Home(screen::Home::new());
                    Task::none()
                }
                manage_emulators::Action::EditEmulator(id) => {
                    self.screen = Screen::ManageEmulators(screen::ManageEmulators::new(Some(id)));
                    Task::none()
                }
            }
        } else {
            Task::none()
        }
    }

    fn update_view_game(&mut self, message: view_game::Message) -> Task<Message> {
        if let Screen::ViewGame(view_game) = &mut self.screen {
            match view_game.update(message) {
                view_game::Action::GoToGames => {
                    self.screen = Screen::Games(screen::Games::new());
                    Task::none()
                }
                view_game::Action::RunWithEmulator(emulator, file, path) => Task::perform(
                    Self::run_with_emulator_async(file, emulator.clone(), path),
                    Message::FinishedRunningWithEmulator,
                ),
            }
        } else {
            Task::none()
        }
    }

    fn update_error(&mut self, message: error_screen::Message) -> Task<Message> {
        if let Screen::Error(error) = &mut self.screen {
            match error.update(message) {
                error_screen::Action::GoHome => {
                    self.screen = Screen::Home(screen::Home::new());
                }
            }
        }
        Task::none()
    }

    fn update_finished_running_emulator(&mut self, result: Result<(), Error>) -> Task<Message> {
        match result {
            Ok(()) => {
                println!("Finished running with emulator");
            }
            Err(_) => println!("Failed to run with emulator"),
        }
        Task::none()
    }

    async fn run_with_emulator_async(
        file: String,
        emulator: model::Emulator,
        path: String,
    ) -> Result<(), Error> {
        let file_path = Path::new(&path).join(&file);
        println!("Running {} with emulator {}", file, emulator.name);
        let mut child = Command::new(&emulator.executable)
            .arg(&file_path)
            .arg(&emulator.arguments)
            .spawn()
            .map_err(|e| Error::IoError(format!("Failed to spawn emulator: {}", e)))?;

        let status = child
            .status()
            .await
            .map_err(|e| Error::IoError(format!("Failed to get status of emulator: {}", e)))?;
        println!("Emulator exited with status: {}", status);
        if !status.success() {
            eprintln!("Emulator failed with status: {}", status);
        }
        println!("Finished running with emulator");

        Ok(())
    }
}
