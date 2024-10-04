use iced::widget::{button, column, text};

#[derive(Debug, Clone)]
pub struct SubScreen2 {
    // ...
}

#[derive(Debug, Clone)]
pub enum Message {
    GoToSubscreen,
}

pub enum Action {
    GoToSubscreen,
    None,
}

impl SubScreen2 {
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
            Message::GoToSubscreen => {
                Action::GoToSubscreen
                // ...
            } // ...
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        // ...
        let title = text("Sub screen 2");
        let go_to_subscreen_button = button("Go to Subscreen 1").on_press(Message::GoToSubscreen);
        column![title, go_to_subscreen_button].into()
    }
}
