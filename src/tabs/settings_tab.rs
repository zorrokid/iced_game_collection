use iced::Task;

use crate::{error::Error, screen::settings_screen::settings_widget};

pub struct SettingsTab {
    settings_widget: settings_widget::SettingsWidget,
}

#[derive(Debug, Clone)]
pub enum Message {
    SettingsWidget(settings_widget::Message),
}

impl SettingsTab {
    pub fn new() -> Result<Self, Error> {
        let settings_widget = settings_widget::SettingsWidget::new()?;

        Ok(Self { settings_widget })
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        if let Message::SettingsWidget(message) = message {
            self.settings_widget
                .update(message)
                .map(Message::SettingsWidget)
        } else {
            Task::none()
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        self.settings_widget.view().map(Message::SettingsWidget)
    }
}
