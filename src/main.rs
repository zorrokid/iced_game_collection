mod database_with_polo;
mod emulator_runner;
mod error;
mod files;
mod model;
mod repository;
mod screen;
mod util;
mod view_model;

use bson::oid::ObjectId;
use emulator_runner::{process_files_for_emulator, run_with_emulator_async};
use error::Error;
use iced::{exit, Task};
use screen::add_release_main;
use screen::error as error_screen;
use screen::games_main;
use screen::home;
use screen::manage_emulators;
use screen::manage_games;
use screen::manage_systems;
use screen::settings_main;
use screen::view_game;
use screen::view_image;
use screen::view_release;

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
    ManageSystems(manage_systems::Message),
    ManageGames(manage_games::Message),
    ManageEmulators(manage_emulators::Message),
    AddReleaseMain(add_release_main::Message),
    GamesMain(games_main::Message),
    FinishedRunningWithEmulator(Result<(), Error>),
    Error(error_screen::Message),
    SettingsMain(settings_main::Message),
    ViewRelease(view_release::Message),
    ViewImage(view_image::Message),
}

impl IcedGameCollection {
    pub fn new() -> (Self, Task<Message>) {
        let home_screen = match home::Home::new() {
            Ok(screen) => Screen::Home(screen),
            Err(e) => Screen::Error(error_screen::Error::new(e)),
        };

        (
            Self {
                screen: home_screen,
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        match &self.screen {
            Screen::Home(home) => home.title(),
            Screen::ManageSystems(add_system) => add_system.title(),
            Screen::ManageGames(manage_games) => manage_games.title(),
            Screen::AddReleaseMain(add_release_main) => add_release_main.title(),
            Screen::GamesMain(games_main) => games_main.title(),
            Screen::ManageEmulators(add_emulator) => add_emulator.title(),
            Screen::Error(error) => error.title(),
            Screen::SettingsMain(settings_main) => settings_main.title(),
            Screen::ViewRelease(view_release) => view_release.title(),
            Screen::ViewImage(view_image) => view_image.title(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ManageSystems(message) => self.update_manage_systems(message),
            Message::ManageGames(message) => self.update_manage_games(message),
            Message::Home(message) => self.update_home(message),
            Message::AddReleaseMain(message) => self.update_add_release(message),
            Message::GamesMain(message) => self.update_games_main(message),
            Message::ManageEmulators(message) => self.update_manage_emulators(message),
            Message::FinishedRunningWithEmulator(result) => {
                self.update_finished_running_emulator(result)
            }
            Message::Error(message) => self.update_error(message),
            Message::SettingsMain(message) => self.update_settings_main(message),
            Message::ViewRelease(message) => self.update_view_release(message),
            Message::ViewImage(message) => self.update_view_image(message),
        }
    }

    fn view(&self) -> iced::Element<Message> {
        match &self.screen {
            Screen::Home(home) => home.view().map(Message::Home),
            Screen::ManageSystems(add_system) => add_system.view().map(Message::ManageSystems),
            Screen::ManageGames(manage_games) => manage_games.view().map(Message::ManageGames),
            Screen::AddReleaseMain(add_release_main) => {
                add_release_main.view().map(Message::AddReleaseMain)
            }
            Screen::GamesMain(games_main) => games_main.view().map(Message::GamesMain),
            Screen::ManageEmulators(add_emulator) => {
                add_emulator.view().map(Message::ManageEmulators)
            }
            Screen::Error(error) => error.view().map(Message::Error),
            Screen::SettingsMain(settings_main) => settings_main.view().map(Message::SettingsMain),
            Screen::ViewRelease(view_release) => view_release.view().map(Message::ViewRelease),
            Screen::ViewImage(view_image) => view_image.view().map(Message::ViewImage),
        }
    }

    fn update_view_image(&mut self, message: view_image::Message) -> Task<Message> {
        if let Screen::ViewImage(view_image) = &mut self.screen {
            match view_image.update(message) {
                view_image::Action::Back => self.try_create_home_screen(),
            }
        } else {
            Task::none()
        }
    }

    fn update_view_release(&mut self, message: view_release::Message) -> Task<Message> {
        if let Screen::ViewRelease(view_release) = &mut self.screen {
            match view_release.update(message) {
                view_release::Action::Back => self.try_create_home_screen(),
                view_release::Action::Run(task) => task.map(Message::ViewRelease),
                view_release::Action::None => Task::none(),
                view_release::Action::RunWithEmulator(options) => Task::perform(
                    run_with_emulator_async(options),
                    Message::FinishedRunningWithEmulator,
                ),
                view_release::Action::Error(error) => {
                    self.screen = Screen::Error(screen::Error::new(error));
                    Task::none()
                }
                view_release::Action::ViewImage(file_path) => {
                    self.screen = Screen::ViewImage(screen::ViewImage::new(file_path));
                    Task::none()
                }
            }
        } else {
            Task::none()
        }
    }

    fn update_settings_main(&mut self, message: settings_main::Message) -> Task<Message> {
        if let Screen::SettingsMain(settings_main) = &mut self.screen {
            match settings_main.update(message) {
                settings_main::Action::Back => self.try_create_home_screen(),
                settings_main::Action::None => Task::none(),
                settings_main::Action::Run(task) => task.map(Message::SettingsMain),
            }
        } else {
            Task::none()
        }
    }

    fn update_manage_systems(&mut self, message: manage_systems::Message) -> Task<Message> {
        if let Screen::ManageSystems(add_system) = &mut self.screen {
            match add_system.update(message) {
                manage_systems::Action::SystemSubmitted | manage_systems::Action::SystemDeleted => {
                    self.handle_navigate_to_manage_systems(None)
                }
                manage_systems::Action::None => Task::none(),
                manage_systems::Action::GoHome => self.try_create_home_screen(),
                manage_systems::Action::EditSystem(id) => {
                    self.handle_navigate_to_manage_systems(Some(id))
                }
                manage_systems::Action::Error(error) => {
                    self.screen = Screen::Error(screen::Error::new(error));
                    Task::none()
                }
            }
        } else {
            Task::none()
        }
    }

    fn update_manage_games(&mut self, message: manage_games::Message) -> Task<Message> {
        if let Screen::ManageGames(manage_games) = &mut self.screen {
            match manage_games.update(message) {
                manage_games::Action::Back => self.try_create_home_screen(),
                _ => Task::none(),
            }
        } else {
            Task::none()
        }
    }

    fn handle_navigate_to_manage_systems(&mut self, id: Option<ObjectId>) -> Task<Message> {
        match screen::ManageSystems::new(id) {
            Ok(screen) => self.screen = Screen::ManageSystems(screen),
            Err(e) => {
                self.screen = Screen::Error(screen::Error::new(e));
            }
        }
        Task::none()
    }

    fn update_home(&mut self, message: home::Message) -> Task<Message> {
        if let Screen::Home(home) = &mut self.screen {
            match home.update(message) {
                home::Action::ViewGames => {
                    match games_main::GamesMain::new() {
                        Ok(screen) => {
                            self.screen = Screen::GamesMain(screen);
                        }
                        Err(e) => {
                            self.screen = Screen::Error(screen::Error::new(e));
                        }
                    }
                    Task::none()
                }
                home::Action::ManageSystems => self.handle_navigate_to_manage_systems(None),
                home::Action::ManageGames => {
                    match screen::manage_games::ManageGames::new(None) {
                        Ok(screen) => {
                            self.screen = Screen::ManageGames(screen);
                        }
                        Err(e) => {
                            self.screen = Screen::Error(screen::Error::new(e));
                        }
                    }
                    Task::none()
                }
                home::Action::AddRelease => {
                    match add_release_main::AddReleaseMain::new(None) {
                        Ok(screen) => {
                            self.screen = Screen::AddReleaseMain(screen);
                        }
                        Err(e) => {
                            self.screen = Screen::Error(screen::Error::new(e));
                        }
                    }
                    Task::none()
                }
                home::Action::Exit => exit(),
                home::Action::ManageEmulators => {
                    let screen = screen::ManageEmulators::new(None);
                    match screen {
                        Ok(screen) => {
                            self.screen = Screen::ManageEmulators(screen);
                        }
                        Err(e) => {
                            self.screen = Screen::Error(screen::Error::new(e));
                        }
                    }
                    Task::none()
                }
                home::Action::ManageSettings => {
                    let screen = screen::SettingsMain::new();
                    match screen {
                        Ok(screen) => {
                            self.screen = Screen::SettingsMain(screen);
                        }
                        Err(e) => {
                            self.screen = Screen::Error(screen::Error::new(e));
                        }
                    }
                    Task::none()
                }
            }
        } else {
            Task::none()
        }
    }

    fn update_add_release(&mut self, message: add_release_main::Message) -> Task<Message> {
        if let Screen::AddReleaseMain(add_release_main) = &mut self.screen {
            match add_release_main.update(message) {
                add_release_main::Action::Back | add_release_main::Action::ReleaseSubmitted => {
                    self.try_create_home_screen()
                }
                add_release_main::Action::Run(task) => task.map(Message::AddReleaseMain),
                add_release_main::Action::None => Task::none(),
                /*add_release_main::Action::RunWithEmulator(options) => Task::perform(
                    run_with_emulator_async(options),
                    Message::FinishedRunningWithEmulator,
                ),*/
                add_release_main::Action::Error(error) => {
                    self.screen = Screen::Error(screen::Error::new(error));
                    Task::none()
                }
            }
        } else {
            Task::none()
        }
    }

    fn update_games_main(&mut self, message: games_main::Message) -> Task<Message> {
        if let Screen::GamesMain(games_main) = &mut self.screen {
            match games_main.update(message) {
                games_main::Action::Back => self.try_create_home_screen(),
                games_main::Action::RunWithEmulator(options) => {
                    match process_files_for_emulator(&options) {
                        Ok(_) => {}
                        Err(e) => {
                            self.screen = Screen::Error(screen::Error::new(e));
                            return Task::none();
                        }
                    }
                    Task::perform(
                        run_with_emulator_async(options),
                        Message::FinishedRunningWithEmulator,
                    )
                }
                games_main::Action::Run(task) => task.map(Message::GamesMain),
                games_main::Action::None => Task::none(),
                games_main::Action::Error(error) => {
                    self.screen = Screen::Error(screen::Error::new(error));
                    Task::none()
                }
                games_main::Action::ViewImage(file_path) => {
                    self.screen = Screen::ViewImage(screen::ViewImage::new(file_path));
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
                    let screen = screen::ManageEmulators::new(None);
                    match screen {
                        Ok(screen) => {
                            self.screen = Screen::ManageEmulators(screen);
                        }
                        Err(e) => {
                            self.screen = Screen::Error(screen::Error::new(e));
                        }
                    }
                    Task::none()
                }
                manage_emulators::Action::None => Task::none(),
                manage_emulators::Action::GoHome => self.try_create_home_screen(),
                manage_emulators::Action::EditEmulator(id) => {
                    let screen = screen::ManageEmulators::new(Some(id));
                    match screen {
                        Ok(screen) => {
                            self.screen = Screen::ManageEmulators(screen);
                        }
                        Err(e) => {
                            self.screen = Screen::Error(screen::Error::new(e));
                        }
                    }
                    Task::none()
                }
                manage_emulators::Action::Error(error) => {
                    self.screen = Screen::Error(screen::Error::new(error));
                    Task::none()
                }
            }
        } else {
            Task::none()
        }
    }

    fn try_create_home_screen(&mut self) -> Task<Message> {
        match home::Home::new() {
            Ok(screen) => {
                self.screen = Screen::Home(screen);
            }
            Err(e) => {
                self.screen = Screen::Error(screen::Error::new(e));
            }
        }
        Task::none()
    }

    fn update_error(&mut self, message: error_screen::Message) -> Task<Message> {
        if let Screen::Error(error) = &mut self.screen {
            match error.update(message) {
                error_screen::Action::GoHome => self.try_create_home_screen(),
            }
        } else {
            Task::none()
        }
    }

    fn update_finished_running_emulator(&mut self, result: Result<(), Error>) -> Task<Message> {
        match result {
            Ok(()) => {
                println!("Finished running with emulator");
            }
            Err(_) => println!("Failed to run with emulator"),
        }
        // TODO: clean up temporary files
        Task::none()
    }
}
