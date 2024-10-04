use iced::widget::{button, column, text, text_input, Column};
use iced::Element;

#[derive(Debug, Clone)]
pub struct SubScreen {
    name: String,
    releases: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    GoToSubscreen2,
    GoHome,
    NameChanged(String),
}

pub enum Action {
    GoToSubscreen2,
    GoHome,
    NameChanged(String),
    None,
}

impl SubScreen {
    pub fn new(name: String, releases: Vec<String>) -> Self {
        Self { name, releases }
    }

    pub fn title(&self) -> String {
        "Add Game".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::GoToSubscreen2 => Action::GoToSubscreen2,
            Message::GoHome => Action::GoHome,
            Message::NameChanged(name) => {
                self.name = name.clone();
                Action::NameChanged(name)
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let title = text("Sub screen 1");
        let name_input_field = text_input("Enter name", &self.name).on_input(Message::NameChanged);

        let releases_list = self
            .releases
            .iter()
            .map(|release| text(release).into())
            .collect::<Vec<Element<Message>>>();

        let add_release_button =
            button("Add release (Go to Subscreen 2)").on_press(Message::GoToSubscreen2);
        let go_home_button = button("Go Home").on_press(Message::GoHome);

        column![
            title,
            Column::with_children(releases_list),
            name_input_field,
            add_release_button,
            go_home_button
        ]
        .into()
    }
}
