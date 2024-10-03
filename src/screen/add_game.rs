use iced::task::Task;
use iced::widget::{button, column, pick_list, text, text_input};
use iced_game_collection::model::{Game, System};

pub struct AddGame {
    pub name: String,
    pub systems: Vec<System>,
    pub selected_system: Option<System>,
}

#[derive(Debug, Clone)]
pub enum Message {
    NameChanged(String),
    SystemSelected(System),
    Submit,
}

pub enum Action {
    SubmitGame(Game),
    None,
}

impl AddGame {
    pub fn new(name: String, systems: Vec<System>) -> (Self, Task<Message>) {
        (
            Self {
                name,
                systems,
                selected_system: None,
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        format!("Add Game {}", self.name)
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::NameChanged(name) => {
                self.name = name;
                Action::None
            }
            Message::Submit => Action::SubmitGame(Game {
                id: 0,
                name: self.name.clone(),
            }),
            Message::SystemSelected(system) => {
                self.selected_system = Some(system);
                Action::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let header = text("Add game").size(50);
        let name_input_field = text_input("Enter name", &self.name).on_input(Message::NameChanged);
        let systems_select = pick_list(
            self.systems.as_slice(),
            self.selected_system.as_ref(),
            Message::SystemSelected,
        );
        let add_button = button("Add Game").on_press(Message::Submit);
        column![header, name_input_field, systems_select, add_button].into()
    }
}
