use crate::model::Game;
use iced::widget::{button, column, text, text_input, Column};
use iced::Element;

#[derive(Debug, Clone)]
pub struct AddGameMainScreen {
    // maybe add game main would hold the game and here only the fields being edited
    // each change would send a message to the main screen to update the game
    // submit would only sent action without payload
    game: Game,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddRelease,
    GoHome,
    NameChanged(String),
    SubmitGame,
}

pub enum Action {
    AddRelease,
    GoHome,
    NameChanged(String),
    SubmitGame,
}

impl AddGameMainScreen {
    pub fn new(game: Game) -> Self {
        Self { game }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::AddRelease => Action::AddRelease,
            Message::GoHome => Action::GoHome,
            Message::NameChanged(name) => {
                self.game.name = name.clone();
                Action::NameChanged(name)
            }
            Message::SubmitGame => Action::SubmitGame,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let back_button = button("Back").on_press(Message::GoHome);
        let name_input_field =
            text_input("Enter name", &self.game.name).on_input(Message::NameChanged);
        let releases_list = self
            .game
            .releases
            .iter()
            .map(|release| text(release.to_string()).into())
            .collect::<Vec<Element<Message>>>();
        let add_release_button = button("Add release").on_press(Message::AddRelease);
        let submit_game_button = button("Submit").on_press(Message::SubmitGame);

        column![
            back_button,
            Column::with_children(releases_list),
            name_input_field,
            add_release_button,
            submit_game_button,
        ]
        .into()
    }
}
