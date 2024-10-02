use iced::widget::text;

pub struct AddGame {
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    NameChanged(String),
    Submit,
}

impl AddGame {
    pub fn new() -> Self {
        Self {
            name: String::new(),
        }
    }

    pub fn title(&self) -> String {
        "Add Game".to_string()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::NameChanged(name) => {
                self.name = name;
            }
            Message::Submit => {
                // Add game to database
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        text("Add Game").size(50).into()
    }
}
