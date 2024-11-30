use crate::database::Database;
use crate::model::{Game, GameListModel};
use iced::widget::{button, column, row, text, text_input, Column};
use iced::Element;

#[derive(Debug, Clone)]
pub struct ManageGames {
    games: Vec<GameListModel>,
    game: Game,
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
}

impl ManageGames {
    pub fn new(edit_game: Option<Game>) -> Self {
        let db = Database::get_instance();
        let games = db.read().unwrap().to_game_list_model();
        Self {
            game: match edit_game {
                Some(game) => game,
                None => Game::default(),
            },
            games,
        }
    }

    pub fn title(&self) -> String {
        "Manage Games".to_string()
    }

    fn update_games(&mut self) {
        let db = Database::get_instance();
        self.games = db.read().unwrap().to_game_list_model();
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Back => Action::Back,
            Message::SubmitGame => {
                let db = Database::get_instance();
                db.write()
                    .unwrap()
                    .add_or_update_game_new(self.game.clone());
                self.update_games();
                Action::GameSubmitted
            }
            Message::DeleteGame(id) => {
                let db = Database::get_instance();
                db.write().unwrap().delete_game(&id);
                self.update_games();
                Action::GameDeleted
            }
            Message::EditGame(id) => {
                let db = Database::get_instance();
                self.game = db.read().unwrap().get_game(&id).unwrap();
                Action::None
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
