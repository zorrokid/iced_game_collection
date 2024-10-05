mod model;
mod screen;

use iced::Task;
use model::{Game, System};
use screen::add_game_main;
use screen::add_system;
use screen::games;
use screen::home;

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
    games: Vec<Game>,
    systems: Vec<System>,
}

#[derive(Debug, Clone)]
enum Message {
    Home(home::Message),
    Games(games::Message),
    AddSystem(add_system::Message),
    AddGameMain(add_game_main::Message),
}

impl IcedGameCollection {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                screen: Screen::Home(home::Home::new()),
                games: vec![],
                systems: vec![],
            },
            // TODO: here coscreen::Games::new(self.games.clone())uld be a task to load games from a database
            Task::none(),
        )
    }

    fn title(&self) -> String {
        match &self.screen {
            Screen::Home(home) => home.title(),
            Screen::Games(games) => games.title(),
            Screen::AddSystem(add_system) => add_system.title(),
            Screen::AddGameMain(add_game_main) => add_game_main.title(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AddSystem(add_system_message) => {
                if let Screen::AddSystem(add_system) = &mut self.screen {
                    let action = add_system.update(add_system_message);
                    match action {
                        add_system::Action::SubmitSystem(system) => {
                            self.systems.push(system);
                            self.screen = Screen::Home(screen::Home::new());
                            Task::none()
                        }
                        add_system::Action::None => Task::none(),
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
                            self.screen = Screen::Games(screen::Games::new(self.games.clone()));
                            Task::none()
                        }
                        home::Action::AddSystem => {
                            self.screen =
                                Screen::AddSystem(screen::AddSystem::new(self.systems.clone()));
                            Task::none()
                        }
                        home::Action::AddGameMain => {
                            self.screen =
                                Screen::AddGameMain(screen::AddGameMain::new(self.systems.clone()));
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
                        games::Action::None => Task::none(),
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
                            self.games.push(game);
                            self.screen = Screen::Games(screen::Games::new(self.games.clone()));
                            Task::none()
                        }
                        add_game_main::Action::None => Task::none(),
                    }
                } else {
                    Task::none()
                }
            }
        }
    }

    fn view(&self) -> iced::Element<Message> {
        match &self.screen {
            Screen::Home(home) => home.view().map(Message::Home),
            Screen::Games(games) => games.view().map(Message::Games),
            Screen::AddSystem(add_system) => add_system.view().map(Message::AddSystem),
            Screen::AddGameMain(add_game_main) => add_game_main.view().map(Message::AddGameMain),
        }
    }
}
