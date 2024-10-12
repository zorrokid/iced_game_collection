use crate::model::{max_id, Game, Release};
use iced::widget::{button, column, text, text_input, Column};
use iced::Element;

#[derive(Debug, Clone)]
pub struct AddGameMainScreen {
    name: String,
    releases: Vec<Release>,
    games: Vec<Game>,
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
    None,
    SubmitGame(Game),
}

impl AddGameMainScreen {
    pub fn new(name: String, releases: Vec<Release>, games: Vec<Game>) -> Self {
        Self {
            name,
            releases,
            games,
        }
    }

    pub fn title(&self) -> String {
        "Add Game".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::AddRelease => Action::AddRelease,
            Message::GoHome => Action::GoHome,
            Message::NameChanged(name) => {
                self.name = name.clone();
                Action::NameChanged(name)
            }
            Message::SubmitGame => Action::SubmitGame(Game {
                id: max_id(&self.games),
                name: self.name.clone(),
                releases: self.releases.clone(),
            }),
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let title = text("Sub screen 1");
        let name_input_field = text_input("Enter name", &self.name).on_input(Message::NameChanged);

        let releases_list = self
            .releases
            .iter()
            .map(|release| text(release.to_string()).into())
            .collect::<Vec<Element<Message>>>();

        let add_release_button =
            button("Add release (Go to Subscreen 2)").on_press(Message::AddRelease);
        let go_home_button = button("Go Home").on_press(Message::GoHome);

        let submit_game_button = button("Submit Game").on_press(Message::SubmitGame);

        column![
            title,
            go_home_button,
            Column::with_children(releases_list),
            name_input_field,
            add_release_button,
            submit_game_button,
        ]
        .into()
    }
}
