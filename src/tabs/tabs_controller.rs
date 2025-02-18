use iced::Task;

use crate::error::Error;

use super::{add_release_tab, games_tab, home_tab, settings_tab};

#[derive(Debug, Clone, PartialEq)]
pub enum Tab {
    Home,
    Settings,
    Games,
    AddRelease,
}

#[derive(Debug, Clone)]
pub enum Message {
    Home(home_tab::Message),
    Settings(settings_tab::Message),
    Games(games_tab::Message),
    AddRelease(add_release_tab::Message),
}

pub struct TabsController {
    current_tab: Tab,
    home_tab: home_tab::HomeTab,
    settings_tab: settings_tab::SettingsTab,
    games_tab: games_tab::GamesTab,
    add_release_tab: add_release_tab::AddReleaseTab,
}

impl TabsController {
    pub fn new(selected_tab: Option<Tab>) -> Result<Self, Error> {
        let settings_tab = settings_tab::SettingsTab::new()?;
        Ok(Self {
            current_tab: selected_tab.unwrap_or(Tab::Home),
            home_tab: home_tab::HomeTab::new(),
            settings_tab,
            games_tab: games_tab::GamesTab::new(),
            add_release_tab: add_release_tab::AddReleaseTab::new(None),
        })
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Home(message) => self.home_tab.update(message).map(Message::Home),
            Message::Settings(message) => self.settings_tab.update(message).map(Message::Settings),
            Message::Games(message) => {
                if let games_tab::Message::EditRelease(id) = message {
                    self.add_release_tab = add_release_tab::AddReleaseTab::new(Some(id));
                    self.current_tab = Tab::AddRelease;
                }
                self.games_tab.update(message).map(Message::Games)
            }
            Message::AddRelease(message) => {
                println!(
                    "tabs_controller: Add release message received: {:?}",
                    message
                );
                self.add_release_tab
                    .update(message)
                    .map(Message::AddRelease)
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        match self.current_tab {
            Tab::Home => self.home_tab.view().map(Message::Home),
            Tab::Settings => self.settings_tab.view().map(Message::Settings),
            Tab::Games => self.games_tab.view().map(Message::Games),
            Tab::AddRelease => self.add_release_tab.view().map(Message::AddRelease),
        }
    }

    pub fn switch_to_tab(&mut self, tab: Tab) -> Task<Message> {
        match tab {
            Tab::Games => self.games_tab = games_tab::GamesTab::new(),
            Tab::AddRelease => self.add_release_tab = add_release_tab::AddReleaseTab::new(None),
            _ => {}
        }
        self.current_tab = tab;
        Task::none()
    }
}
