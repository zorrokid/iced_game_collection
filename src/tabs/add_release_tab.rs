use iced::{
    widget::{button, column, container, pick_list, row, text, text_input},
    Task,
};

use crate::{
    database_with_polo::DatabaseWithPolo,
    model::model::{Game, System},
    repository::repository::SystemReadRepository,
};

use super::widgets::{add_game_widget, add_system_widget};

pub struct AddReleaseTab {
    games: Vec<Game>,
    selected_game: Option<Game>,
    add_game_widget: add_game_widget::AddGame,
    add_system_widget: add_system_widget::AddSystem,
    adding_game: bool,
    adding_system: bool,
    release_name: String,
    systems: Vec<System>,
    selected_system: Option<System>,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddGame(add_game_widget::Message),
    AddSystem(add_system_widget::Message),
    GameSelected(Game),
    StartAddingGame,
    StopAddingGame,
    StartAddingSystem,
    StopAddingSystem,
    ReleaseNameUpdated(String),
    SystemSelected(System),
}

impl AddReleaseTab {
    pub fn new() -> Self {
        let db = DatabaseWithPolo::get_instance();
        let games = db.get_all_games().unwrap_or_else(|err| {
            println!("Failed to get games {:?}", err);
            vec![]
        });
        let systems = db.get_all_systems().unwrap_or_else(|err| {
            println!("Failed to get systems {:?}", err);
            vec![]
        });
        Self {
            add_game_widget: add_game_widget::AddGame::new(),
            add_system_widget: add_system_widget::AddSystem::new(),
            games,
            adding_game: false,
            adding_system: false,
            selected_game: None,
            release_name: "".to_string(),
            systems,
            selected_system: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AddGame(message) => {
                match self.add_game_widget.update(message) {
                    add_game_widget::Action::GameAdded(game) => {
                        self.games.push(game);
                        self.adding_game = false;
                    }
                    add_game_widget::Action::CancelAddGame => {
                        self.adding_game = false;
                    }
                    add_game_widget::Action::None => {}
                }
                Task::none()
            }
            Message::AddSystem(message) => {
                match self.add_system_widget.update(message) {
                    add_system_widget::Action::SystemAdded(system) => {
                        self.systems.push(system);
                        self.adding_system = false;
                    }
                    add_system_widget::Action::CancelAddSystem => {
                        self.adding_system = false;
                    }
                    add_system_widget::Action::None => {}
                }
                Task::none()
            }
            Message::StartAddingGame => {
                self.adding_game = true;
                Task::none()
            }
            Message::StopAddingGame => {
                self.adding_game = false;
                Task::none()
            }
            Message::StartAddingSystem => {
                self.adding_system = true;
                Task::none()
            }
            Message::StopAddingSystem => {
                self.adding_system = false;
                Task::none()
            }
            Message::GameSelected(game) => {
                self.selected_game = Some(game);
                Task::none()
            }
            Message::ReleaseNameUpdated(name) => {
                self.release_name = name;
                Task::none()
            }
            Message::SystemSelected(system) => {
                self.selected_system = Some(system);
                Task::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let release_name_input =
            text_input("Release Name", &self.release_name).on_input(Message::ReleaseNameUpdated);
        let games_row = row![
            self.create_games_dropdown(),
            button("Add Game")
                .on_press_maybe((!self.adding_game).then(|| Message::StartAddingGame)),
        ];

        let systems_row = row![
            self.create_systems_dropdown(),
            button("Add System")
                .on_press_maybe((!self.adding_system).then(|| Message::StartAddingSystem)),
        ];

        let add_game_row = if self.adding_game {
            self.create_add_game_container()
        } else {
            row![].into()
        };

        let add_system_row = if self.adding_system {
            self.create_add_system_container()
        } else {
            row![].into()
        };

        column![
            release_name_input,
            games_row,
            add_game_row,
            systems_row,
            add_system_row,
            button("Submit"),
        ]
        .into()
    }

    fn create_add_game_container(&self) -> iced::Element<Message> {
        let content = self.add_game_widget.view().map(Message::AddGame);
        container(content)
            .padding(10)
            .style(container::rounded_box)
            .into()
    }

    fn create_add_system_container(&self) -> iced::Element<Message> {
        let content = self.add_system_widget.view().map(Message::AddSystem);
        container(content)
            .padding(10)
            .style(container::rounded_box)
            .into()
    }

    fn create_games_dropdown(&self) -> iced::Element<Message> {
        pick_list(
            self.games.clone(),
            self.selected_game.clone(),
            Message::GameSelected,
        )
        .into()
    }

    fn create_systems_dropdown(&self) -> iced::Element<Message> {
        pick_list(
            self.systems.clone(),
            self.selected_system.clone(),
            Message::SystemSelected,
        )
        .into()
    }
}
