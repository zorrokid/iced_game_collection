use iced::widget::{button, column, text};

#[derive(Debug, Clone)]
pub struct SubScreen {
    // ...
}

#[derive(Debug, Clone)]
pub enum Message {
    GoToSubscreen2,
    GoHome,
}

pub enum Action {
    GoToSubscreen2,
    GoHome,
    None,
}

impl SubScreen {
    pub fn new() -> Self {
        Self {
            // ...
        }
    }

    pub fn title(&self) -> String {
        "Add Game".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::GoToSubscreen2 => Action::GoToSubscreen2,
            Message::GoHome => Action::GoHome,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let title = text("Sub screen 1");
        let go_to_subscreen2_button = button("Go to Subscreen 2").on_press(Message::GoToSubscreen2);
        let go_home_button = button("Go Home").on_press(Message::GoHome);
        column![title, go_to_subscreen2_button, go_home_button].into()
    }
}
