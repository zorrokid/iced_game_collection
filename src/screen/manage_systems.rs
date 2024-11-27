use crate::database::Database;
use crate::model::{init_new_system, System};
use crate::util::directory::create_directory_name;
use iced::widget::{button, column, row, text, text_input, Column};

#[derive(Debug, Clone)]
pub struct ManageSystems {
    pub system: System,
    pub systems: Vec<System>,
}

#[derive(Debug, Clone)]
pub enum Message {
    NameChanged(String),
    GoHome,
    Submit,
    EditSystem(i32),
    DeleteSystem(i32),
    Clear,
}

pub enum Action {
    GoHome,
    None,
    EditSystem(i32),
    SystemDeleted,
    SystemSubmitted,
}

impl ManageSystems {
    pub fn new(edit_system_id: Option<i32>) -> Self {
        let db = Database::get_instance();
        let systems = db.read().unwrap().get_systems();
        let edit_system = edit_system_id.and_then(|id| db.read().unwrap().get_system(id));
        Self {
            system: match edit_system {
                Some(system) => system,
                None => init_new_system(&systems),
            },
            systems,
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
            Message::Submit => match &mut self.system.name {
                name if name.is_empty() => Action::None,
                _ => {
                    self.system.directory = create_directory_name(&self.system.name);
                    let db = Database::get_instance();
                    db.write()
                        .unwrap()
                        .add_or_update_system(self.system.clone());
                    Action::SystemSubmitted
                }
            },
            Message::GoHome => Action::GoHome,
            Message::EditSystem(id) => Action::EditSystem(id),
            Message::DeleteSystem(id) => {
                let db = Database::get_instance();
                db.write().unwrap().delete_system(id);
                Action::SystemDeleted
            }
            Message::Clear => {
                self.system = init_new_system(&self.systems);
                Action::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let name_input_field =
            text_input("Enter name", &self.system.name).on_input(Message::NameChanged);
        let main_buttons = row![
            button("Submit").on_press(Message::Submit),
            button("Clear").on_press(Message::Clear)
        ];
        let systems_list = self
            .systems
            .iter()
            .map(|system| {
                row![
                    text(system.to_string()).width(iced::Length::Fixed(300.0)),
                    button("Edit").on_press(Message::EditSystem(system.id)),
                    button("Delete").on_press(Message::DeleteSystem(system.id)),
                ]
                .into()
            })
            .collect::<Vec<iced::Element<Message>>>();

        let back_button = button("Back").on_press(Message::GoHome);
        column![
            back_button,
            name_input_field,
            main_buttons,
            Column::with_children(systems_list)
        ]
        .into()
    }
}
