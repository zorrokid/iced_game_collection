use iced::Task;

use crate::error::Error;

use super::{games_tab, home_tab, settings_tab};

#[derive(Debug, Clone)]
pub enum Tab {
    Home,
    Settings,
    Games,
}

#[derive(Debug, Clone)]
pub enum Message {
    Home(home_tab::Message),
    Settings(settings_tab::Message),
    Games(games_tab::Message),
}

pub struct TabsController {
    current_tab: Tab,
    home_tab: home_tab::HomeTab,
    settings_tab: settings_tab::SettingsTab,
    games_tab: games_tab::GamesTab,
}

impl TabsController {
    pub fn new(selected_tab: Option<Tab>) -> Result<Self, Error> {
        let settings_tab = settings_tab::SettingsTab::new()?;
        Ok(Self {
            current_tab: selected_tab.unwrap_or(Tab::Home),
            home_tab: home_tab::HomeTab::new(),
            settings_tab,
            games_tab: games_tab::GamesTab::new(),
        })
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Home(message) => self.home_tab.update(message).map(Message::Home),
            Message::Settings(message) => self.settings_tab.update(message).map(Message::Settings),
            Message::Games(message) => self.games_tab.update(message).map(Message::Games),
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        match self.current_tab {
            Tab::Home => self.home_tab.view().map(Message::Home),
            Tab::Settings => self.settings_tab.view().map(Message::Settings),
            Tab::Games => self.games_tab.view().map(Message::Games),
        }
    }

    pub fn switch_to_tab(&mut self, tab: Tab) -> Task<Message> {
        self.current_tab = tab;
        Task::none()
    }
}
