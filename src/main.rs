mod error;
mod files;
mod model;
mod screen;

use async_process::Command;
use async_std::fs::File as AsyncFile;
use async_std::io::ReadExt;
use async_std::io::WriteExt;
use async_std::path::Path;
use error::Error;
use iced::{exit, Task};
use model::Collection;
use screen::add_release_main;
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
    AddReleaseMain(add_release_main::Message),
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
            Task::perform(load_collection_async(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        match &self.screen {
            Screen::Home(home) => home.title(),
            Screen::Games(games) => games.title(),
            Screen::ManageSystems(add_system) => add_system.title(),
            Screen::AddReleaseMain(add_release_main) => add_release_main.title(),
            Screen::ManageEmulators(add_emulator) => add_emulator.title(),
            Screen::ViewGame(view_game) => view_game.title(),
            Screen::Error(error) => error.title(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ManageSystems(message) => self.update_manage_systems(message),
            Message::Home(message) => self.update_home(message),
            Message::Games(message) => self.update_games(message),
            Message::AddReleaseMain(message) => self.update_add_release(message),
            Message::Loaded(result) => self.update_loaded(result),
            Message::CollectionSavedOnExit(result) => self.update_collection_saved_on_exit(result),
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
            let action = add_system.update(message);
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

    fn update_home(&mut self, message: home::Message) -> Task<Message> {
        if let Screen::Home(home) = &mut self.screen {
            let action = home.update(message);
            match action {
                home::Action::ViewGames => {
                    self.screen =
                        Screen::Games(screen::Games::new(self.collection.to_game_list_model()));
                    Task::none()
                }
                home::Action::ManageSystems => {
                    self.screen = Screen::ManageSystems(screen::ManageSystems::new(
                        self.collection.systems.clone(),
                        None,
                    ));
                    Task::none()
                }
                home::Action::AddRelease => {
                    self.screen = Screen::AddReleaseMain(screen::AddReleaseMain::new(
                        self.collection.games_new.clone(),
                        None,
                        self.collection.systems.clone(),
                    ));
                    Task::none()
                }
                home::Action::Exit => Task::perform(
                    save_collection_async(self.collection.clone()),
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

    fn update_games(&mut self, message: games::Message) -> Task<Message> {
        if let Screen::Games(games) = &mut self.screen {
            let action = games.update(message);
            match action {
                games::Action::GoHome => {
                    let home = home::Home::new();
                    self.screen = Screen::Home(home);
                    Task::none()
                }
                games::Action::ViewGame(id) => {
                    let game = self.collection.get_game(id);
                    let releases = self.collection.get_releases_with_game(id);
                    if let Some(game) = game {
                        let view_game = view_game::ViewGame::new(
                            game,
                            self.collection.emulators.clone(),
                            releases,
                            self.collection.systems.clone(),
                        );
                        self.screen = Screen::ViewGame(view_game);
                    }
                    Task::none()
                }
                games::Action::EditGame(id) => {
                    let edit_game = self.collection.games.iter().find(|g| g.id == id).unwrap();

                    // TODO
                    Task::none()
                }
                games::Action::DeleteGame(id) => {
                    // TODO: before game can be deleted, files related to releases must be deleted first
                    //  - only in case relase has only this game
                    self.collection.delete_game(id);
                    self.screen =
                        Screen::Games(screen::Games::new(self.collection.to_game_list_model()));
                    Task::none()
                }
            }
        } else {
            Task::none()
        }
    }

    fn update_add_release(&mut self, message: add_release_main::Message) -> Task<Message> {
        if let Screen::AddReleaseMain(add_release_main) = &mut self.screen {
            let action = add_release_main.update(message);
            match action {
                add_release_main::Action::Back => {
                    self.screen = Screen::Home(screen::Home::new());
                    Task::none()
                }
                add_release_main::Action::SubmitRelease(release) => {
                    self.collection.add_or_update_release(release);
                    self.screen = Screen::Home(screen::Home::new());
                    Task::none()
                }
                add_release_main::Action::Run(task) => task.map(Message::AddReleaseMain),
                add_release_main::Action::None => Task::none(),
                add_release_main::Action::Error(e) => {
                    self.screen = Screen::Error(screen::Error::new(e));
                    Task::none()
                }
                add_release_main::Action::SubmitGame(game) => {
                    self.collection.add_or_update_game_new(game);
                    Task::none()
                }
                add_release_main::Action::DeleteSystem(id) => {
                    self.collection.delete_system(id);
                    Task::none()
                }
                add_release_main::Action::SubmitSystem(system) => {
                    self.collection.add_or_update_system(system);
                    Task::none()
                }
            }
        } else {
            Task::none()
        }
    }

    fn update_manage_emulators(&mut self, message: manage_emulators::Message) -> Task<Message> {
        if let Screen::ManageEmulators(add_emulator) = &mut self.screen {
            let action = add_emulator.update(message);
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

    fn update_view_game(&mut self, message: view_game::Message) -> Task<Message> {
        if let Screen::ViewGame(view_game) = &mut self.screen {
            let action = view_game.update(message);
            match action {
                view_game::Action::GoToGames => {
                    self.screen =
                        Screen::Games(screen::Games::new(self.collection.to_game_list_model()));
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

    fn update_loaded(&mut self, result: Result<Collection, Error>) -> Task<Message> {
        match result {
            Ok(games) => {
                self.collection = games;
                Task::none()
            }
            Err(err) => {
                eprintln!("Failed to load collection: {}", err);
                Task::none()
            }
        }
    }

    fn update_collection_saved_on_exit(&mut self, result: Result<(), Error>) -> Task<Message> {
        if let Err(e) = result {
            match e {
                Error::IoError(e) => eprintln!("Failed to save collection: {}", e),
                _ => eprintln!("Failed to save collection"),
            }
        }
        exit()
    }

    fn update_error(&mut self, message: error_screen::Message) -> Task<Message> {
        if let Screen::Error(error) = &mut self.screen {
            let action = error.update(message);
            match action {
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

async fn load_collection_async() -> Result<Collection, Error> {
    let mut file = AsyncFile::open(COLLECTION_FILE_NAME)
        .await
        .map_err(|e| Error::IoError(format!("Failed to open {}: {}", COLLECTION_FILE_NAME, e)))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .await
        .map_err(|e| Error::IoError(format!("Failed to read {}: {}", COLLECTION_FILE_NAME, e)))?;
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
    let mut file = AsyncFile::create(COLLECTION_FILE_NAME)
        .await
        .map_err(|e| Error::IoError(format!("Failed to create {}: {}", COLLECTION_FILE_NAME, e)))?;
    file.write(json.as_bytes())
        .await
        .map_err(|e| Error::IoError(format!("Failed to write {}: {}", COLLECTION_FILE_NAME, e)))?;
    Ok(())
}
