use bson::oid::ObjectId;
use iced::{
    widget::{button, row, text, Column},
    Length, Task,
};

use crate::{
    model::model::Game,
    view_model::list_models::{get_releases_in_list_model, ReleaseListModel},
};

pub struct ReleasesList {
    game: Option<Game>,
    releases: Vec<ReleaseListModel>,
}

#[derive(Debug, Clone)]
pub enum Message {
    GameSelected(ObjectId),
    ViewRelease(ObjectId),
}

pub enum Action {
    None,
}

impl ReleasesList {
    pub fn new() -> Self {
        Self {
            game: None,
            releases: vec![],
            // Initialize fields here
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::GameSelected(game_id) => {
                let db = crate::database_with_polo::DatabaseWithPolo::get_instance();
                let releases = get_releases_in_list_model(db, &game_id);
                let game = db.get_game(&game_id);
                self.releases = releases.unwrap_or_else(|err| {
                    println!("Failed to get releases list {:?}", err);
                    vec![]
                });
                self.game = game.unwrap_or_else(|err| {
                    println!("Failed to get game {:?}", err);
                    None
                });
                Action::None
            }
            Message::ViewRelease(id) => {
                println!("ViewRelease message received with id: {:?}", id);
                Action::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let releases_list = self
            .releases
            .iter()
            .map(|release| {
                let view_release_button = button("View")
                    .on_press(Message::ViewRelease(release.id))
                    .width(Length::Fixed(100.0));
                let release_row = row![
                    text(&release.name).width(Length::Fixed(100.0)),
                    view_release_button,
                ];

                release_row.into()
            })
            .collect::<Vec<iced::Element<Message>>>();
        Column::with_children(releases_list).into()
    }
}
