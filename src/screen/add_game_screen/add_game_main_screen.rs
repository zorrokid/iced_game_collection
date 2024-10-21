use iced::widget::{button, column, text_input};

#[derive(Debug, Clone)]
pub struct AddGameMainScreen {
    name: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ManageReleases,
    GoHome,
    NameChanged(String),
    SubmitGame,
}

pub enum Action {
    ManageReleases,
    GoHome,
    NameChanged(String),
    SubmitGame,
}

impl AddGameMainScreen {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ManageReleases => Action::ManageReleases,
            Message::GoHome => Action::GoHome,
            Message::NameChanged(name) => {
                self.name = name.clone();
                Action::NameChanged(name)
            }
            Message::SubmitGame => Action::SubmitGame,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let back_button = button("Back").on_press(Message::GoHome);
        let name_input_field = text_input("Enter name", &self.name).on_input(Message::NameChanged);
        let manage_releases_button = button("Manage releases").on_press(Message::ManageReleases);
        let submit_game_button = button("Submit").on_press(Message::SubmitGame);

        column![
            back_button,
            name_input_field,
            manage_releases_button,
            submit_game_button,
        ]
        .into()
    }
}
