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
use model::ToGameListModel;
use screen::add_emulator;
use screen::add_game_main;
use screen::add_system;
use screen::games;
use screen::home;
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
    AddSystem(add_system::Message),
    AddEmulator(add_emulator::Message),
    AddGameMain(add_game_main::Message),
    Loaded(Result<Collection, Error>),
    CollectionSavedOnExit(Result<(), Error>),
    ViewGame(screen::view_game::Message),
    FinishedRunningWithEmulator(Result<(), Error>),
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
            Screen::AddSystem(add_system) => add_system.title(),
            Screen::AddGameMain(add_game_main) => add_game_main.title(),
            Screen::AddEmulator(add_emulator) => add_emulator.title(),
            Screen::ViewGame(view_game) => view_game.title(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AddSystem(add_system_message) => {
                if let Screen::AddSystem(add_system) = &mut self.screen {
                    let action = add_system.update(add_system_message);
                    match action {
                        add_system::Action::SubmitSystem(system) => {
                            self.collection.systems.push(system);
                            self.screen = Screen::Home(screen::Home::new());
                            Task::none()
                        }
                        add_system::Action::None => Task::none(),
                        add_system::Action::GoHome => {
                            self.screen = Screen::Home(screen::Home::new());
                            Task::none()
                        }
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
                        home::Action::AddSystem => {
                            self.screen = Screen::AddSystem(screen::AddSystem::new(
                                self.collection.systems.clone(),
                            ));
                            Task::none()
                        }
                        home::Action::AddGame => {
                            self.screen = Screen::AddGameMain(screen::AddGameMain::new(
                                self.collection.systems.clone(),
                                self.collection.games.clone(),
                            ));
                            Task::none()
                        }
                        home::Action::Exit => Task::perform(
                            Self::save_collection_async(self.collection.clone()),
                            Message::CollectionSavedOnExit,
                        ),
                        home::Action::AddEmulator => {
                            self.screen = Screen::AddEmulator(screen::AddEmulator::new(
                                self.collection.emulators.clone(),
                                self.collection.systems.clone(),
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
                        games::Action::EditGame(_) => Task::none(),
                        games::Action::DeleteGame(id) => {
                            self.collection.games.retain(|g| g.id != id);
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
                            self.collection.games.push(game);
                            self.screen = Screen::Games(screen::Games::new(
                                self.collection.to_game_list_model(),
                            ));
                            Task::none()
                        }
                        add_game_main::Action::Run(task) => task.map(Message::AddGameMain),
                        add_game_main::Action::None => Task::none(),
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
                Err(_) => {
                    eprintln!("Failed to load games");
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
            Message::AddEmulator(add_emulator_message) => {
                if let Screen::AddEmulator(add_emulator) = &mut self.screen {
                    let action = add_emulator.update(add_emulator_message);
                    match action {
                        add_emulator::Action::SubmitEmulator(emulator) => {
                            self.collection.emulators.push(emulator);
                            self.screen = Screen::Home(screen::Home::new());
                            Task::none()
                        }
                        add_emulator::Action::None => Task::none(),
                        add_emulator::Action::GoHome => {
                            self.screen = Screen::Home(screen::Home::new());
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
        }
    }

    fn view(&self) -> iced::Element<Message> {
        match &self.screen {
            Screen::Home(home) => home.view().map(Message::Home),
            Screen::Games(games) => games.view().map(Message::Games),
            Screen::AddSystem(add_system) => add_system.view().map(Message::AddSystem),
            Screen::AddGameMain(add_game_main) => add_game_main.view().map(Message::AddGameMain),
            Screen::AddEmulator(add_emulator) => add_emulator.view().map(Message::AddEmulator),
            Screen::ViewGame(view_game) => view_game.view().map(Message::ViewGame),
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
        // spawn emulator
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
