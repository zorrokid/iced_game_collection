use crate::model::{get_new_id, System};
use iced::widget::{button, column, row, text, text_input, Column};

// TODO: move AddSystem under Add Release?
pub struct ManageSystems {
    pub system: System,
    pub systems: Vec<System>,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    NameChanged(String),
    GoHome,
    Submit,
    EditSystem(i32),
    DeleteSystem(i32),
}

pub enum Action {
    SubmitSystem(System),
    GoHome,
    None,
    EditSystem(i32),
    DeleteSystem(i32),
}

impl ManageSystems {
    pub fn new(systems: Vec<System>, edit_system: Option<System>) -> Self {
        Self {
            system: match edit_system {
                Some(system) => system,
                None => System {
                    id: get_new_id(&systems),
                    name: "".to_string(),
                },
            },
            systems,
            error: None,
        }
    }

    pub fn title(&self) -> String {
        "Manage systems".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::NameChanged(name) => {
                self.system.name = name;
                Action::None
            }
            Message::Submit => {
                if self.system.name.is_empty() {
                    self.error = Some("Name cannot be empty".to_string());
                    return Action::None;
                } else {
                    Action::SubmitSystem(self.system.clone())
                }
            }
            Message::GoHome => Action::GoHome,
            Message::EditSystem(id) => Action::EditSystem(id),
            Message::DeleteSystem(id) => Action::DeleteSystem(id),
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let name_input_field =
            text_input("Enter name", &self.system.name).on_input(Message::NameChanged);
        let add_button = button("Submit").on_press(Message::Submit);
        let systems_list = self
            .systems
            .iter()
            .map(|system| {
                row![
                    text(system.to_string()),
                    button("Edit").on_press(Message::EditSystem(system.id)),
                    button("Delete").on_press(Message::DeleteSystem(system.id)),
                ]
                .into()
            })
            .collect::<Vec<iced::Element<Message>>>();
        let error = if let Some(error) = &self.error {
            text(error)
        } else {
            text("")
        };
        let back_button = button("Back").on_press(Message::GoHome);
        column![
            back_button,
            error,
            name_input_field,
            add_button,
            Column::with_children(systems_list)
        ]
        .into()
    }
}
