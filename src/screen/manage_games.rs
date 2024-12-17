use crate::database_with_polo::DatabaseWithPolo;
use crate::error::Error;
use crate::model::model::{Game, GameListModel};
use iced::widget::{button, column, row, text, text_input, Column};
use iced::Element;

#[derive(Debug, Clone)]
pub struct ManageGames {
    games: Vec<GameListModel>,
    game: Game,
    is_edit: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Back,
    SubmitGame,
    DeleteGame(String),
    EditGame(String),
    NameChanged(String),
    Clear,
}

#[derive(Debug, Clone)]
pub enum Action {
    Back,
    None,
    GameSubmitted,
    GameDeleted,
    Error(Error),
}

impl ManageGames {
    pub fn new(edit_game: Option<Game>) -> Result<Self, Error> {
        let db = DatabaseWithPolo::get_instance();
        let games = db.to_game_list_model()?;
        let is_edit = edit_game.is_some();
        Ok(Self {
            game: match edit_game {
                Some(game) => game,
                None => Game::default(),
            },
            games,
            is_edit,
        })
    }

    pub fn title(&self) -> String {
        "Manage Games".to_string()
    }

    fn update_games(&mut self) -> Result<(), Error> {
        let db = DatabaseWithPolo::get_instance();
        let games = db.to_game_list_model()?;
        self.games = games;
        Ok(())
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Back => Action::Back,
            Message::SubmitGame => {
                let db = DatabaseWithPolo::get_instance();
                let res = match self.is_edit {
                    true => db.update_game(&self.game),
                    false => db.add_game(&self.game),
                };

                match res {
                    Ok(_) => {
                        self.update_games();
                        Action::GameSubmitted
                    }
                    Err(e) => Action::Error(e),
                }
            }
            Message::DeleteGame(id) => {
                let db = DatabaseWithPolo::get_instance();
                match db.delete_game(&id) {
                    Ok(_) => {
                        self.update_games();
                        Action::GameDeleted
                    }
                    Err(e) => Action::Error(e),
                }
            }
            Message::EditGame(id) => {
                let db = DatabaseWithPolo::get_instance();
                match db.get_game(&id) {
                    Ok(game) => match game {
                        Some(game) => {
                            self.game = game;
                            self.is_edit = true;
                            Action::None
                        }
                        None => {
                            Action::Error(Error::DbError(format!("Game with id {} not found", &id)))
                        }
                    },
                    Err(e) => return Action::Error(e),
                }
            }
            Message::NameChanged(name) => {
                self.game.name = name;
                Action::None
            }
            Message::Clear => {
                self.game = Game::default();
                Action::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let back_button = button("Back").on_press(Message::Back);
        let name_input_field =
            text_input("Enter name", &self.game.name).on_input(Message::NameChanged);
        let main_buttons = row![
            button("Submit").on_press(Message::SubmitGame),
            button("Clear").on_press(Message::Clear)
        ];

        let games_list = self
            .games
            .iter()
            .map(|game| {
                row![
                    text(&game.name).width(iced::Length::Fixed(300.0)),
                    button("Edit")
                        .on_press(Message::EditGame(game.id.clone()))
                        .width(iced::Length::Fixed(200.0)),
                    button("Delete")
                        .on_press_maybe(if game.can_delete {
                            Some(Message::DeleteGame(game.id.clone()))
                        } else {
                            None
                        })
                        .width(iced::Length::Fixed(200.0))
                ]
                .into()
            })
            .collect::<Vec<Element<Message>>>();
        column![
            back_button,
            name_input_field,
            main_buttons,
            Column::with_children(games_list)
        ]
        .into()
    }
}
