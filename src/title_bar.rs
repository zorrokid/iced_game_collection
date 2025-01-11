use iced::{
    widget::{button, row},
    Element,
};

#[derive(Debug, Clone)]
pub enum Tab {
    Home,
    Settings,
}

#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(Tab),
}

pub struct TitleBar {
    active_tab: Tab,
}

impl TitleBar {
    pub fn new() -> Self {
        Self {
            active_tab: Tab::Home,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(index) => {
                self.active_tab = index;
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let home_button = button("Home").on_press(Message::TabSelected(Tab::Home));
        let settings_button = button("Settings").on_press(Message::TabSelected(Tab::Settings));
        row![home_button, settings_button].into()
    }
}
