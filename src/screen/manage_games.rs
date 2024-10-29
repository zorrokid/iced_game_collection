use crate::database::get_new_id;
use crate::model::Game;
use iced::widget::{button, column, row, text, text_input, Column};
use iced::Element;

#[derive(Debug, Clone)]
pub struct ManageGames {
    games: Vec<Game>,
    game: Game,
}

#[derive(Debug, Clone)]
pub enum Message {
    Back,
    SubmitGame,
    DeleteGame(i32),
    EditGame(i32),
    NameChanged(String),
}

#[derive(Debug, Clone)]
pub enum Action {
    Back,
    SubmitGame(Game),
    DeleteGame(i32),
    EditGame(i32),
    None,
}

impl ManageGames {
    pub fn new(edit_game: Option<Game>) -> Self {
        let db = crate::database::Database::get_instance();
        let games = db.read().unwrap().get_games();
        Self {
            game: match edit_game {
                Some(game) => game,
                None => Game {
                    id: get_new_id(&games),
                    name: "".to_string(),
                },
            },
            games,
        }
    }

    pub fn title(&self) -> String {
        "Manage Games".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Back => Action::Back,
            Message::SubmitGame => Action::SubmitGame(self.game.clone()),
            Message::DeleteGame(id) => Action::DeleteGame(id),
            Message::EditGame(id) => Action::EditGame(id),
            Message::NameChanged(name) => {
                self.game.name = name;
                Action::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let back_button = button("Back").on_press(Message::Back);
        let title = text("Manage Games");
        let name_input_field =
            text_input("Enter name", &self.game.name).on_input(Message::NameChanged);
        let submit_button = button("Submit").on_press(Message::SubmitGame);

        let games_list = self
            .games
            .iter()
            .map(|game| {
                row![
                    text(&game.name).width(iced::Length::Fixed(300.0)),
                    button("Edit")
                        .on_press(Message::EditGame(game.id))
                        .width(iced::Length::Fixed(200.0)),
                    button("Delete")
                        .on_press(Message::DeleteGame(game.id))
                        .width(iced::Length::Fixed(200.0))
                ]
                .into()
            })
            .collect::<Vec<Element<Message>>>();
        column![
            title,
            back_button,
            name_input_field,
            submit_button,
            Column::with_children(games_list)
        ]
        .into()
    }
}
