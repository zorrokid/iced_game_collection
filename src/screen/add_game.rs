use iced::task::Task;
use iced::widget::{button, column, text, text_input};
use iced_game_collection::model::Game;

pub struct AddGame {
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    NameChanged(String),
    Submit,
}

pub enum Action {
    SubmitGame(Game),
    None,
}

impl AddGame {
    pub fn new(name: String) -> (Self, Task<Message>) {
        (Self { name }, Task::none())
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
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let header = text("Add game").size(50);
        let name_input_field = text_input("Enter name", &self.name).on_input(Message::NameChanged);
        let add_button = button("Add Game").on_press(Message::Submit);
        column![header, name_input_field, add_button].into()
    }
}
