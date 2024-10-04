use iced::widget::{button, column, text, text_input};

pub struct Home {}

#[derive(Debug, Clone)]
pub enum Message {
    AddGame,
    ViewGames,
    AddSystem,
    AddGameMain,
}

pub enum Action {
    AddGame,
    ViewGames,
    AddSystem,
    AddGameMain,
    None,
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
            Message::AddGame => Action::AddGame,
            Message::ViewGames => Action::ViewGames,
            Message::AddSystem => Action::AddSystem,
            Message::AddGameMain => Action::AddGameMain,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let add_button = button("Add Game").on_press(Message::AddGame);
        let header = text("Welcome to Iced Game Collection").size(50);

        let view_games_button = button("View Games").on_press(Message::ViewGames);
        let add_system_button = button("Add System").on_press(Message::AddSystem);
        let go_to_add_game_main_button = button("Add Game Main").on_press(Message::AddGameMain);

        column![
            header,
            add_button,
            view_games_button,
            add_system_button,
            go_to_add_game_main_button
        ]
        .into()
    }
}
