use crate::model::{Release, System};
use iced::widget::{button, column, pick_list, text, text_input};

use super::AddGame;

pub struct AddRelease {
    pub name: String,
    pub systems: Vec<System>,
    pub selected_system: Option<System>,
    pub add_game: AddGame,
}

#[derive(Debug, Clone)]
pub enum Message {
    NameChanged(String),
    SystemSelected(System),
    Submit,
}

pub enum Action {
    SubmitRelease(Release, AddGame),
    None,
}

impl AddRelease {
    pub fn new(systems: Vec<System>, add_game: AddGame) -> Self {
        Self {
            name: "".to_string(),
            systems,
            selected_system: None,
            add_game,
        }
    }

    pub fn title(&self) -> String {
        format!("Add Release {}", self.name)
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::NameChanged(name) => {
                self.name = name;
                Action::None
            }
            Message::Submit => {
                if let Some(system) = &self.selected_system {
                    Action::SubmitRelease(
                        Release {
                            id: 0,
                            name: self.name.clone(),
                            system_id: system.id,
                        },
                        self.add_game.clone(),
                    )
                } else {
                    Action::None
                }
            }
            Message::SystemSelected(system) => {
                self.selected_system = Some(system);
                Action::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let header = text("Add release").size(50);
        let name_input_field = text_input("Enter name", &self.name).on_input(Message::NameChanged);
        let systems_select = pick_list(
            self.systems.as_slice(),
            self.selected_system.as_ref(),
            Message::SystemSelected,
        );
        let add_button = button("Add Release").on_press(Message::Submit);
        column![header, name_input_field, systems_select, add_button].into()
    }
}
