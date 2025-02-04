use iced::{widget::{button, column, container, pick_list, row, text, text_input}, Task};

use crate::{database_with_polo::DatabaseWithPolo, model::model::Game};

use super::widgets::add_game_widget;

pub struct AddReleaseTab {
    games: Vec<Game>,
    selected_game: Option<Game>,
    add_game_widget: add_game_widget::AddGame,
    adding_game: bool,
    release_name: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddGame(add_game_widget::Message),
    GameSelected(Game),
    StartAddingGame,
    StopAddingGame,
    ReleaseNameUpdated(String),
}

impl AddReleaseTab {
    pub fn new() -> Self {
        let db = DatabaseWithPolo::get_instance();
        let games = db.get_all_games().unwrap_or_else(|err| {
            println!("Failed to get games {:?}", err);
            vec![]
        });
        Self {
            add_game_widget: add_game_widget::AddGame::new(),
            games,
            adding_game: false,
            selected_game: None,
            release_name: "".to_string(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AddGame(message) => {
                match self.add_game_widget.update(message) {
                    add_game_widget::Action::GameAdded(game) => {
                        self.games.push(game);
                        self.adding_game = false;
                    }
                    add_game_widget::Action::CancelAddGame => {
                        self.adding_game = false;
                    }
                    add_game_widget::Action::None => {}
                }
                Task::none()
            }
            Message::StartAddingGame => {
                self.adding_game = true;
                Task::none()
            }
            Message::StopAddingGame => {
                self.adding_game = false;
                Task::none()
            }
            Message::GameSelected(game) => {
                self.selected_game = Some(game);
                Task::none()
            }
            Message::ReleaseNameUpdated(name) => {
                self.release_name = name;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let release_name_input = text_input("Release Name", &self.release_name)
            .on_input(Message::ReleaseNameUpdated);
        let games_row = row![
            self.create_games_dropdown(),
            button("Add Game").on_press_maybe((!self.adding_game).then(|| Message::StartAddingGame)),
        ];

        let add_game_row = if self.adding_game {
            self.create_add_game_container()
            //self.add_game_widget.view().map(Message::AddGame)
        } else {
            row![].into()
        };
        
        column![
            release_name_input,
            games_row,
            add_game_row,
            button("Submit"),
        ].into()

    }

    fn create_add_game_container(&self) -> iced::Element<Message> {
        let content = 
            self.add_game_widget.view().map(Message::AddGame);
        container(content).padding(10).style(container::rounded_box).into()
    }

    fn create_games_dropdown(&self) -> iced::Element<Message> {
        pick_list(
            self.games.clone(),
            self.selected_game.clone(),
            Message::GameSelected,
        ).into()

    }
}


