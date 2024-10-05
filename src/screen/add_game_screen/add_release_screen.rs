use crate::model::Release;
use iced::widget::{button, column, text, text_input};

#[derive(Debug, Clone)]
pub struct AddReleaseScreen {
    name: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    GoToSubscreen,
    ReleaseAdded(String),
}

pub enum Action {
    ReleaseAdded(Release),
    None,
}

impl AddReleaseScreen {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
        }
    }

    pub fn title(&self) -> String {
        "Add Game".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::GoToSubscreen => {
                // Action::GoToSubscreen(self.name.clone()),
                let release = Release {
                    id: 0,
                    name: self.name.clone(),
                    system_id: 0,
                };
                Action::ReleaseAdded(release)
            }
            Message::ReleaseAdded(name) => {
                self.name = name.clone();
                Action::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        // ...
        let title = text("Sub screen 2");
        let release_name_input_field =
            text_input("Enter release name", &self.name).on_input(Message::ReleaseAdded);

        let submit_button =
            button("Submit (and go to Subscreen 1)").on_press(Message::GoToSubscreen);
        column![title, release_name_input_field, submit_button].into()
    }
}
