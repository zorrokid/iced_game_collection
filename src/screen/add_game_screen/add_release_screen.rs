use crate::model::{Release, System};
use iced::widget::{button, column, pick_list, text, text_input};

#[derive(Debug, Clone)]
pub struct AddReleaseScreen {
    pub name: String,
    pub systems: Vec<System>,
    pub selected_system: Option<System>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Submit,
    NameChanged(String),
    SystemSelected(System),
    GoBack,
}

pub enum Action {
    ReleaseAdded(Release),
    GoBack,
    None,
}

impl AddReleaseScreen {
    pub fn new(systems: Vec<System>) -> Self {
        Self {
            name: "".to_string(),
            systems,
            selected_system: None,
        }
    }

    pub fn title(&self) -> String {
        "Add Game".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Submit => {
                if let Some(system) = &self.selected_system {
                    Action::ReleaseAdded(Release {
                        id: 0, // TODO: maybe use Option, set id when saving to db?
                        name: self.name.clone(),
                        system: system.clone(),
                    })
                } else {
                    Action::None
                }
            }
            Message::NameChanged(name) => {
                self.name = name.clone();
                Action::None
            }
            Message::SystemSelected(system) => {
                self.selected_system = Some(system);
                Action::None
            }
            Message::GoBack => Action::GoBack,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let title = text("Sub screen 2");
        let release_name_input_field =
            text_input("Enter release name", &self.name).on_input(Message::NameChanged);
        let systems_select = pick_list(
            self.systems.as_slice(),
            self.selected_system.as_ref(),
            Message::SystemSelected,
        );
        let back_button = button("Back").on_press(Message::GoBack);

        let submit_button = button("Submit").on_press(Message::Submit);
        column![
            back_button,
            title,
            release_name_input_field,
            systems_select,
            submit_button
        ]
        .into()
    }
}
