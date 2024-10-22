mod error;
mod model;
mod screen;

use async_process::Command;
use async_std::fs::File as AsyncFile;
use async_std::io::ReadExt;
use async_std::io::WriteExt;
use error::Error;
use iced::{exit, Task};
use model::Collection;
use screen::add_game_main;
use screen::error as error_screen;
use screen::games;
use screen::home;
use screen::manage_emulators;
use screen::manage_systems;
use screen::view_game;
use serde_json::to_string_pretty;

use crate::screen::Screen;

const COLLECTION_FILE_NAME: &str = "games.json";

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
    collection: Collection,
}

#[derive(Debug, Clone)]
enum Message {
    Home(home::Message),
    Games(games::Message),
    ManageSystems(manage_systems::Message),
    ManageEmulators(manage_emulators::Message),
    AddGameMain(add_game_main::Message),
    Loaded(Result<Collection, Error>),
    CollectionSavedOnExit(Result<(), Error>),
    ViewGame(screen::view_game::Message),
    FinishedRunningWithEmulator(Result<(), Error>),
    Error(error_screen::Message),
}

impl IcedGameCollection {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                screen: Screen::Home(home::Home::new()),
                collection: Collection::default(),
            },
            Task::perform(Self::load_collection_async(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        match &self.screen {
            Screen::Home(home) => home.title(),
            Screen::Games(games) => games.title(),
            Screen::ManageSystems(add_system) => add_system.title(),
            Screen::AddGameMain(add_game_main) => add_game_main.title(),
            Screen::ManageEmulators(add_emulator) => add_emulator.title(),
            Screen::ViewGame(view_game) => view_game.title(),
            Screen::Error(error) => error.title(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ManageSystems(add_system_message) => {
                if let Screen::ManageSystems(add_system) = &mut self.screen {
                    let action = add_system.update(add_system_message);
                    match action {
                        manage_systems::Action::SubmitSystem(system) => {
                            self.collection.add_or_update_system(system);
                            self.screen = Screen::ManageSystems(screen::ManageSystems::new(
                                self.collection.systems.clone(),
                                None,
                            ));
                            Task::none()
                        }
                        manage_systems::Action::None => Task::none(),
                        manage_systems::Action::GoHome => {
                            self.screen = Screen::Home(screen::Home::new());
                            Task::none()
                        }
                        manage_systems::Action::EditSystem(id) => {
                            self.screen = Screen::ManageSystems(screen::ManageSystems::new(
                                self.collection.systems.clone(),
                                self.collection.get_system(id),
                            ));
                            Task::none()
                        }
                        manage_systems::Action::DeleteSystem(id) => {
                            self.collection.delete_system(id);
                            self.screen = Screen::ManageSystems(screen::ManageSystems::new(
                                self.collection.systems.clone(),
                                None,
                            ));
                            Task::none()
                        }
                        manage_systems::Action::Run(task) => task.map(Message::ManageSystems),
                    }
                } else {
                    Task::none()
                }
            }
            Message::Home(home_message) => {
                if let Screen::Home(home) = &mut self.screen {
                    let action = home.update(home_message);
                    match action {
                        home::Action::ViewGames => {
                            self.screen = Screen::Games(screen::Games::new(
                                self.collection.to_game_list_model(),
                            ));
                            Task::none()
                        }
                        home::Action::ManageSystems => {
                            self.screen = Screen::ManageSystems(screen::ManageSystems::new(
                                self.collection.systems.clone(),
                                None,
                            ));
                            Task::none()
                        }
                        home::Action::AddGame => {
                            self.screen = Screen::AddGameMain(screen::AddGameMain::new(
                                self.collection.systems.clone(),
                                self.collection.games.clone(),
                                None,
                            ));
                            Task::none()
                        }
                        home::Action::Exit => Task::perform(
                            Self::save_collection_async(self.collection.clone()),
                            Message::CollectionSavedOnExit,
                        ),
                        home::Action::ManageEmulators => {
                            self.screen = Screen::ManageEmulators(screen::ManageEmulators::new(
                                self.collection.emulators.clone(),
                                self.collection.systems.clone(),
                                None,
                            ));
                            Task::none()
                        }
                    }
                } else {
                    Task::none()
                }
            }
            Message::Games(games_message) => {
                if let Screen::Games(games) = &mut self.screen {
                    let action = games.update(games_message);
                    match action {
                        games::Action::GoHome => {
                            let home = home::Home::new();
                            self.screen = Screen::Home(home);
                            Task::none()
                        }
                        games::Action::ViewGame(id) => {
                            let game = self.collection.games.iter().find(|g| g.id == id).unwrap();
                            let view_game = view_game::ViewGame::new(
                                game.clone(),
                                self.collection.emulators.clone(),
                            );
                            self.screen = Screen::ViewGame(view_game);
                            Task::none()
                        }
                        games::Action::EditGame(id) => {
                            let edit_game =
                                self.collection.games.iter().find(|g| g.id == id).unwrap();

                            self.screen = Screen::AddGameMain(screen::AddGameMain::new(
                                self.collection.systems.clone(),
                                self.collection.games.clone(),
                                Some(edit_game.clone()),
                            ));
                            Task::none()
                        }
                        games::Action::DeleteGame(id) => {
                            self.collection.delete_game(id);
                            self.screen = Screen::Games(screen::Games::new(
                                self.collection.to_game_list_model(),
                            ));
                            Task::none()
                        }
                    }
                } else {
                    Task::none()
                }
            }
            Message::AddGameMain(add_game_main_message) => {
                if let Screen::AddGameMain(add_game_main) = &mut self.screen {
                    let action = add_game_main.update(add_game_main_message);
                    match action {
                        add_game_main::Action::GoHome => {
                            self.screen = Screen::Home(screen::Home::new());
                            Task::none()
                        }
                        add_game_main::Action::SubmitGame(game) => {
                            self.collection.add_or_update_game(game);
                            self.screen = Screen::Games(screen::Games::new(
                                self.collection.to_game_list_model(),
                            ));
                            Task::none()
                        }
                        add_game_main::Action::Run(task) => task.map(Message::AddGameMain),
                        add_game_main::Action::None => Task::none(),
                        add_game_main::Action::Error(e) => {
                            self.screen = Screen::Error(screen::Error::new(e));
                            Task::none()
                        }
                    }
                } else {
                    Task::none()
                }
            }
            Message::Loaded(collection) => match collection {
                Ok(games) => {
                    self.collection = games;
                    Task::none()
                }
                Err(err) => {
                    eprintln!("Failed to load collection: {}", err);
                    Task::none()
                }
            },
            Message::CollectionSavedOnExit(result) => {
                if let Err(e) = result {
                    match e {
                        Error::IoError(e) => eprintln!("Failed to save collection: {}", e),
                        _ => eprintln!("Failed to save collection"),
                    }
                }
                exit()
            }
            Message::ManageEmulators(add_emulator_message) => {
                if let Screen::ManageEmulators(add_emulator) = &mut self.screen {
                    let action = add_emulator.update(add_emulator_message);
                    match action {
                        manage_emulators::Action::SubmitEmulator(emulator) => {
                            self.collection.add_or_update_emulator(emulator);
                            self.screen = Screen::ManageEmulators(screen::ManageEmulators::new(
                                self.collection.emulators.clone(),
                                self.collection.systems.clone(),
                                None,
                            ));
                            Task::none()
                        }
                        manage_emulators::Action::None => Task::none(),
                        manage_emulators::Action::GoHome => {
                            self.screen = Screen::Home(screen::Home::new());
                            Task::none()
                        }
                        manage_emulators::Action::DeleteEmulator(id) => {
                            self.collection.delete_emulator(id);
                            self.screen = Screen::ManageEmulators(screen::ManageEmulators::new(
                                self.collection.emulators.clone(),
                                self.collection.systems.clone(),
                                None,
                            ));
                            Task::none()
                        }
                        manage_emulators::Action::EditEmulator(id) => {
                            self.screen = Screen::ManageEmulators(screen::ManageEmulators::new(
                                self.collection.emulators.clone(),
                                self.collection.systems.clone(),
                                self.collection.get_emulator(id),
                            ));
                            Task::none()
                        }
                    }
                } else {
                    Task::none()
                }
            }
            Message::ViewGame(view_game_message) => {
                if let Screen::ViewGame(view_game) = &mut self.screen {
                    let action = view_game.update(view_game_message);
                    match action {
                        view_game::Action::GoToGames => {
                            self.screen = Screen::Games(screen::Games::new(
                                self.collection.to_game_list_model(),
                            ));
                            Task::none()
                        }
                        view_game::Action::RunWithEmulator(emulator, file) => {
                            println!("Running with emulator: {}", file);
                            Task::perform(
                                Self::run_with_emulator_async(file, emulator.clone()),
                                Message::FinishedRunningWithEmulator,
                            )
                        }
                    }
                } else {
                    Task::none()
                }
            }
            Message::FinishedRunningWithEmulator(result) => {
                match result {
                    Ok(()) => {
                        println!("Finished running with emulator");
                    }
                    Err(_) => println!("Failed to run with emulator"),
                }

                Task::none()
            }
            Message::Error(error_message) => {
                if let Screen::Error(error) = &mut self.screen {
                    let action = error.update(error_message);
                    match action {
                        error_screen::Action::GoHome => {
                            self.screen = Screen::Home(screen::Home::new());
                        }
                    }
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> iced::Element<Message> {
        match &self.screen {
            Screen::Home(home) => home.view().map(Message::Home),
            Screen::Games(games) => games.view().map(Message::Games),
            Screen::ManageSystems(add_system) => add_system.view().map(Message::ManageSystems),
            Screen::AddGameMain(add_game_main) => add_game_main.view().map(Message::AddGameMain),
            Screen::ManageEmulators(add_emulator) => {
                add_emulator.view().map(Message::ManageEmulators)
            }
            Screen::ViewGame(view_game) => view_game.view().map(Message::ViewGame),
            Screen::Error(error) => error.view().map(Message::Error),
        }
    }

    async fn load_collection_async() -> Result<Collection, Error> {
        let mut file = AsyncFile::open(COLLECTION_FILE_NAME).await.map_err(|e| {
            Error::IoError(format!("Failed to open {}: {}", COLLECTION_FILE_NAME, e))
        })?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await.map_err(|e| {
            Error::IoError(format!("Failed to read {}: {}", COLLECTION_FILE_NAME, e))
        })?;
        let games = serde_json::from_str(&contents).map_err(|e| {
            Error::IoError(format!(
                "Failed to deserialize {}: {}",
                COLLECTION_FILE_NAME, e
            ))
        })?;
        Ok(games)
    }

    async fn save_collection_async(collection: Collection) -> Result<(), Error> {
        let json = to_string_pretty(&collection)
            .map_err(|e| Error::IoError(format!("Failed to serialize games: {}", e)))?;
        let mut file = AsyncFile::create(COLLECTION_FILE_NAME).await.map_err(|e| {
            Error::IoError(format!("Failed to create {}: {}", COLLECTION_FILE_NAME, e))
        })?;
        file.write(json.as_bytes()).await.map_err(|e| {
            Error::IoError(format!("Failed to write {}: {}", COLLECTION_FILE_NAME, e))
        })?;
        Ok(())
    }

    async fn run_with_emulator_async(file: String, emulator: model::Emulator) -> Result<(), Error> {
        println!("Running {} with emulator {}", file, emulator.name);
        let mut child = Command::new(&emulator.executable)
            .arg(&file)
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
