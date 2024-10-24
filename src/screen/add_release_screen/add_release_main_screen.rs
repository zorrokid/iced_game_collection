use crate::error::Error;
use crate::files::pick_file;
use crate::model::{Game, Release, System};
use async_std::path::PathBuf;
use iced::widget::{button, column, pick_list, text, text_input, Column};
use iced::{Element, Task};

#[derive(Debug, Clone)]
pub struct AddReleaseMainScreen {
    games: Vec<Game>,
    selected_game: Option<Game>,
    release: Release,
    systems: Vec<System>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ManageGames,
    ManageSystems,
    Back,
    GameSelected(Game),
    NameChanged(String),
    SystemSelected(System),
    SelectFile,
    FileAdded(Result<PathBuf, Error>),
    Submit,
}

pub enum Action {
    ManageGames,
    ManageSystems,
    Back,
    GameSelected(Game),
    NameChanged(String),
    None,
    SystemSelected(System),
    Run(Task<Message>),
    AddFile(String),
    Submit(Release),
}

impl AddReleaseMainScreen {
    pub fn new(games: Vec<Game>, release: Release, systems: Vec<System>) -> Self {
        Self {
            games,
            selected_game: None,
            release,
            systems,
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ManageGames => Action::ManageGames,
            Message::ManageSystems => Action::ManageSystems,
            Message::Back => Action::Back,
            Message::GameSelected(game) => Action::GameSelected(game),
            Message::NameChanged(name) => Action::NameChanged(name),
            Message::SystemSelected(system) => Action::SystemSelected(system),
            Message::SelectFile => {
                let selected_system = self
                    .systems
                    .iter()
                    .find(|system| system.id == self.release.system_id);
                if let Some(system) = selected_system {
                    let source_path = system.roms_source_path.clone();
                    let destination_path = system.roms_destination_path.clone();
                    // We need to wrap the Task in an Action, because with Action we can pass the Task back to the main update-function which
                    // returns a Task<Message> which is then passed back to the iced runtime. Iced runtime passes the Message with the result from the
                    // Task back to the update function.
                    Action::Run(Task::perform(
                        pick_file(source_path, destination_path),
                        Message::FileAdded,
                    ))
                } else {
                    Action::None
                }
            }
            Message::FileAdded(result) => {
                if let Ok(path) = result {
                    if let Some(file_name) = path
                        .file_name()
                        .and_then(|os_str| os_str.to_str().map(|s| s.to_string()))
                    {
                        return Action::AddFile(file_name);
                    }
                }
                Action::None
            }
            Message::Submit => Action::Submit(self.release.clone()),
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let title = text("Add Release Main Screen");
        let back_button = button("Back").on_press(Message::Back);
        let release_name_input_field =
            text_input("Enter release name", &self.release.name).on_input(Message::NameChanged);

        let selected_games_title = text("Selected Games:");

        let selected_games_list = self
            .release
            .games
            .iter()
            .map(|game_id| {
                let game = self.games.iter().find(|game| game.id == *game_id).unwrap();
                text(&game.name).into()
            })
            .collect::<Vec<Element<Message>>>();

        let manage_games_button: button::Button<'_, Message> = button("Manage Games")
            .width(iced::Length::Fixed(200.0))
            .on_press(Message::ManageGames);

        let selected_system = self
            .systems
            .iter()
            .find(|system| system.id == self.release.system_id);

        let systems_select = pick_list(
            self.systems.as_slice(),
            selected_system,
            Message::SystemSelected,
        );
        let files_list = self
            .release
            .files
            .iter()
            .map(|file| text(file).into())
            .collect::<Vec<iced::Element<Message>>>();
        let add_file_button = button("Add File").on_press(Message::SelectFile);
        let manage_systems_button = button("Manage Systems")
            .width(iced::Length::Fixed(200.0))
            .on_press(Message::ManageSystems);
        let available_games: Vec<Game> = self
            .games
            .iter()
            .filter(|g| !self.release.games.contains(&g.id))
            .cloned()
            .collect();
        let game_picker = pick_list(
            available_games,
            self.selected_game.clone(),
            Message::GameSelected,
        );
        let submit_button = button("Submit").on_press(Message::Submit);
        column![
            title,
            release_name_input_field,
            selected_games_title,
            Column::with_children(selected_games_list),
            back_button,
            game_picker,
            manage_games_button,
            systems_select,
            manage_systems_button,
            add_file_button,
            Column::with_children(files_list),
            submit_button
        ]
        .into()
    }
}
