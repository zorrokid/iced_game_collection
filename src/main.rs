mod screen;

use iced::Task;
use iced_game_collection::model::{Game, System};
use screen::add_game;
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
}

impl IcedGameCollection {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                screen: Screen::Home(home::Home::new()),
                games: vec![],
                systems: vec![],
            },
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
                            let (view_games, task) = screen::Games::new(self.games.clone());
                            self.screen = Screen::Games(view_games);
                            return task.map(Message::Games);
                        }
                        add_game::Action::None => {}
                    }
                }
                Task::none()
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
            Message::Home(home_message) => {
                if let Screen::Home(home) = &mut self.screen {
                    let action = home.update(home_message);
                    match action {
                        home::Action::AddGame(name) => {
                            let (add_game, task) = screen::AddGame::new(name, self.systems.clone());
                            self.screen = Screen::AddGame(add_game);
                            task.map(Message::AddGame)
                        }
                        home::Action::ViewGames => {
                            let (view_games, task) = screen::Games::new(self.games.clone());
                            self.screen = Screen::Games(view_games);
                            task.map(Message::Games)
                        }
                        home::Action::AddSystem => {
                            self.screen = Screen::AddSystem(screen::AddSystem::new());
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
        }
    }

    fn view(&self) -> iced::Element<Message> {
        match &self.screen {
            Screen::Home(home) => home.view().map(Message::Home),
            Screen::AddGame(add_game) => add_game.view().map(Message::AddGame),
            Screen::Games(games) => games.view().map(Message::Games),
            Screen::GameDetails(game_details) => game_details.view().map(Message::GameDetails),
            Screen::AddSystem(add_system) => add_system.view().map(Message::AddSystem),
        }
    }
}
