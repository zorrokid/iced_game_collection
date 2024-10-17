use iced::widget::{button, column};

pub struct Home {}

#[derive(Debug, Clone)]
pub enum Message {
    ViewGames,
    AddSystem,
    AddGame,
    ManageEmulators,
    Exit,
}

pub enum Action {
    ViewGames,
    AddSystem,
    AddGame,
    ManageEmulators,
    Exit,
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
            Message::AddGame => Action::AddGame,
            Message::ManageEmulators => Action::ManageEmulators,
            Message::Exit => Action::Exit,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let view_games_button = button("View Games").on_press(Message::ViewGames);
        let add_system_button = button("Add System").on_press(Message::AddSystem);
        let add_game_button = button("Add Game").on_press(Message::AddGame);
        let add_emulator_button = button("Manage emulators").on_press(Message::ManageEmulators);
        let exit_button = button("Exit").on_press(Message::Exit);

        column![
            view_games_button,
            add_system_button,
            add_game_button,
            add_emulator_button,
            exit_button
        ]
        .into()
    }
}
