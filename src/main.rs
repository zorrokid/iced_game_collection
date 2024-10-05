mod model;
mod screen;

use iced::Task;
use model::{Game, System};
use screen::add_game;
use screen::add_game_main;
use screen::add_release;
use screen::add_system;
use screen::game_details;
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
    AddGame(add_game::Message),
    Home(home::Message),
    Games(games::Message),
    GameDetails(game_details::Message),
    AddSystem(add_system::Message),
    AddRelease(add_release::Message),
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
            Screen::Home(_) => "Home".to_string(),
            Screen::AddGame(add_game) => add_game.title(),
            Screen::Games(games) => games.title(),
            Screen::GameDetails(game_details) => game_details.title(),
            Screen::AddSystem(add_system) => add_system.title(),
            Screen::AddRelease(add_release) => add_release.title(),
            Screen::AddGameMain(add_game_main) => add_game_main.title(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AddGame(add_game_message) => {
                if let Screen::AddGame(add_game) = &mut self.screen {
                    let action = add_game.update(add_game_message);
                    match action {
                        add_game::Action::SubmitGame(game) => {
                            self.games.push(game);
                            self.screen = Screen::Games(screen::Games::new(self.games.clone()));
                            Task::none()
                        }
                        add_game::Action::None => Task::none(),
                        add_game::Action::AddRelease(add_game_state) => {
                            self.screen = Screen::AddRelease(screen::AddRelease::new(
                                self.systems.clone(),
                                add_game_state,
                            ));
                            Task::none()
                        }
                    }
                } else {
                    Task::none()
                }
            }

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
            // TODO: AddGame should be like main for other screens
            Message::AddRelease(add_release_message) => {
                if let Screen::AddRelease(add_release) = &mut self.screen {
                    let action = add_release.update(add_release_message);
                    match action {
                        add_release::Action::SubmitRelease(release, add_game) => {
                            // should I pass the add game state to add release and then to add game to be restored?
                            self.screen =
                                Screen::AddGame(screen::AddGame::new(Some(add_game.clone())));
                            Task::none()
                        }
                        add_release::Action::None => Task::none(),
                    }
                } else {
                    Task::none()
                }
            }
            Message::Home(home_message) => {
                if let Screen::Home(home) = &mut self.screen {
                    let action = home.update(home_message);
                    match action {
                        home::Action::AddGame => {
                            self.screen = Screen::AddGame(screen::AddGame::new(None));
                            Task::none()
                        }
                        home::Action::ViewGames => {
                            self.screen = Screen::Games(screen::Games::new(self.games.clone()));
                            Task::none()
                        }
                        home::Action::AddSystem => {
                            self.screen = Screen::AddSystem(screen::AddSystem::new());
                            Task::none()
                        }
                        home::Action::AddGameMain => {
                            self.screen = Screen::AddGameMain(screen::AddGameMain::new());
                            Task::none()
                        }
                        home::Action::None => Task::none(),
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
            Message::GameDetails(game_details_message) => Task::none(),
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
            Screen::AddGame(add_game) => add_game.view().map(Message::AddGame),
            Screen::Games(games) => games.view().map(Message::Games),
            Screen::GameDetails(game_details) => game_details.view().map(Message::GameDetails),
            Screen::AddSystem(add_system) => add_system.view().map(Message::AddSystem),
            Screen::AddRelease(add_release) => add_release.view().map(Message::AddRelease),
            Screen::AddGameMain(add_game_main) => add_game_main.view().map(Message::AddGameMain),
        }
    }
}
