use bson::oid::ObjectId;
use iced::{
    widget::{row, text},
    Task,
};

use super::widgets::{
    games_list_widget::{self, GamesList},
    release_details_widget::{self, ReleaseDetails},
    releases_list_widget::{self, ReleasesList},
};

pub struct GamesTab {
    games_list: GamesList,
    releases_list: ReleasesList,
    release_details: ReleaseDetails,
    // Add fields here
}

#[derive(Debug, Clone)]
pub enum Message {
    GameSelected(games_list_widget::Message),
    ReleaseSelected(releases_list_widget::Message),
    ShowReleaseDetails(release_details_widget::Message),
    // Add message variants here
}

impl GamesTab {
    pub fn new() -> Self {
        Self {
            games_list: GamesList::new(),
            releases_list: ReleasesList::new(),
            release_details: ReleaseDetails::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::GameSelected(message) => {
                println!("Game selected message received: {:?}", message);
                match message {
                    games_list_widget::Message::ViewGame(game_id) => {
                        println!("Game selected message received with game id: {:?}", game_id);
                        self.releases_list
                            .update(releases_list_widget::Message::GameSelected(game_id));
                        Task::none()
                    }
                }
            } // Handle other messages here
            Message::ReleaseSelected(message) => {
                println!("Release selected message received: {:?}", message);
                match message {
                    releases_list_widget::Message::ViewRelease(release_id) => {
                        println!(
                            "Release selected message received with release id: {:?}",
                            release_id
                        );
                        self.release_details
                            .update(release_details_widget::Message::ReleaseSelected(release_id));
                        Task::none()
                    }
                    releases_list_widget::Message::GameSelected(game_id) => {
                        println!(
                            "Release selected message received with game id: {:?}",
                            game_id
                        );
                        self.releases_list
                            .update(releases_list_widget::Message::GameSelected(game_id));
                        Task::none()
                    }
                }
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        row![
            self.games_list.view().map(Message::GameSelected),
            self.releases_list.view().map(Message::ReleaseSelected),
            self.release_details.view().map(Message::ShowReleaseDetails),
        ]
        .into()
    }
}
