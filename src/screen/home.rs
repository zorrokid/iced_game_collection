use iced::widget::{button, column, text, text_input};

pub struct Home {
    name: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddGame,
    ViewGames,
    NameChanged(String),
    AddSystem,
}

pub enum Action {
    AddGame(String),
    ViewGames,
    AddSystem,
    None,
}

impl Home {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
        }
    }

    pub fn title(&self) -> String {
        "Iced Game Collection".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::AddGame => Action::AddGame(self.name.clone()),
            Message::ViewGames => Action::ViewGames,
            Message::NameChanged(name) => {
                self.name = name;
                Action::None
            }
            Message::AddSystem => Action::AddSystem,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let name_input_field = text_input("Enter name", &self.name).on_input(Message::NameChanged);
        let add_button = button("Add Game").on_press(Message::AddGame);
        let header = text("Welcome to Iced Game Collection").size(50);

        let view_games_button = button("View Games").on_press(Message::ViewGames);
        let add_system_button = button("Add System").on_press(Message::AddSystem);

        column![
            header,
            name_input_field,
            add_button,
            view_games_button,
            add_system_button
        ]
        .into()
    }
}
