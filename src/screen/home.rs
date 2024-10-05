use iced::widget::{button, column, text};

pub struct Home {}

#[derive(Debug, Clone)]
pub enum Message {
    ViewGames,
    AddSystem,
    AddGameMain,
}

pub enum Action {
    ViewGames,
    AddSystem,
    AddGameMain,
}

impl Home {
    pub fn new() -> Self {
        Self {}
    }

    pub fn title(&self) -> String {
        "Iced Game Collection".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ViewGames => Action::ViewGames,
            Message::AddSystem => Action::AddSystem,
            Message::AddGameMain => Action::AddGameMain,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let header = text("Welcome to Iced Game Collection").size(50);

        let view_games_button = button("View Games").on_press(Message::ViewGames);
        let add_system_button = button("Add System").on_press(Message::AddSystem);
        let add_game_button = button("Add Game Main").on_press(Message::AddGameMain);

        column![
            header,
            view_games_button,
            add_system_button,
            add_game_button
        ]
        .into()
    }
}
