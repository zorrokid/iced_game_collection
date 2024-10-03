mod screen;
use iced::Task;
use iced_game_collection::model::Game;
use screen::add_game;
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
}

#[derive(Debug, Clone)]
enum Message {
    AddGame(add_game::Message),
    Home(home::Message),
    Games(games::Message),
    GameDetails(game_details::Message),
}

impl IcedGameCollection {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                screen: Screen::Home(home::Home::new()),
                games: vec![],
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
                            self.screen = Screen::Games(games::Games::new(self.games.clone()));
                        }
                        add_game::Action::None => {}
                    }
                }
                /*print!("Add game message: {:?}", add_game_message);
                if let Screen::AddGame(add_game) = &mut self.screen {
                    let action = add_game.update(add_game_message);
                    match action {
                        home::Action::AddGame => {
                            self.screen = Screen::AddGame(add_game::AddGame::new());*/
                // Add game to database
                /*self.games.push(Game {
                    id: 1,
                    name: add_game.name.clone(),
                });
                self.screen = Screen::Games(games::Games::new(self.games.clone()));*/
                /* }
                    }
                }
                self.screen = Screen::AddGame(add_game::AddGame::new());*/
                Task::none()
            }

            Message::Home(home_message) => {
                if let Screen::Home(home) = &mut self.screen {
                    let action = home.update(home_message);
                    match action {
                        home::Action::AddGame(name) => {
                            let (add_game, task) = screen::AddGame::new(name);
                            self.screen = Screen::AddGame(add_game);
                            task.map(Message::AddGame)
                        }
                        home::Action::None => Task::none(),
                    }
                } else {
                    Task::none()
                }
            }
            Message::Games(games_message) => Task::none(),
            Message::GameDetails(game_details_message) => Task::none(),
        }
    }

    fn view(&self) -> iced::Element<Message> {
        match &self.screen {
            Screen::Home(home) => home.view().map(Message::Home),
            Screen::AddGame(add_game) => add_game.view().map(Message::AddGame),
            Screen::Games(games) => games.view().map(Message::Games),
            Screen::GameDetails(game_details) => game_details.view().map(Message::GameDetails),
        }
    }
}
