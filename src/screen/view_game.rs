use crate::{
    error::Error,
    model::model::{Game, HasOid, Release, System},
};
use bson::oid::ObjectId;
use iced::widget::{button, column, row, text, Column};

// TODO: ViewGame needs to be a main screen with subscreens:
// - view game main screen with list of releases
// - view release screen
// - edit release screen
// - view image screen
#[derive(Debug, Clone)]
pub struct ViewGame {
    game: Game,
    releases: Vec<Release>,
    systems: Vec<System>,
}

#[derive(Debug, Clone)]
pub enum Message {
    GoToGames,
    EditRelease(ObjectId),
    ViewRelease(ObjectId),
}

#[derive(Debug, Clone)]
pub enum Action {
    Back,
    EditRelease(ObjectId),
    ViewRelease(ObjectId),
}

impl ViewGame {
    pub fn new(game_id: ObjectId) -> Result<Self, Error> {
        let db = crate::database_with_polo::DatabaseWithPolo::get_instance();
        let releases = db.get_releases_with_game(&game_id)?;
        let systems = db.get_systems()?;

        let game = db.get_game(&game_id)?;
        match game {
            None => Err(Error::NotFound(format!(
                "Game with id {} not found",
                game_id
            ))),
            Some(game) => Ok(Self {
                game,
                releases,
                systems,
            }),
        }
    }

    pub fn title(&self) -> String {
        format!("{} releases", self.game.name)
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::GoToGames => Action::Back,
            Message::EditRelease(id) => Action::EditRelease(id),
            Message::ViewRelease(id) => Action::ViewRelease(id),
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let title = text(self.game.name.clone()).size(30);

        let releases_list = self
            .releases
            .iter()
            .map(|release| {
                let system = self
                    .systems
                    .iter()
                    .find(|s| {
                        release
                            .system_id
                            .map_or(false, |system_id| s.id() == system_id)
                    })
                    .unwrap();

                let edit_release_button =
                    button("Edit").on_press(Message::EditRelease(release.id()));

                let view_release_button =
                    button("View").on_press(Message::ViewRelease(release.id()));

                let release_row = row![
                    text(release.to_string()),
                    text(system.name.clone()),
                    view_release_button,
                    edit_release_button,
                ];

                release_row.into()
            })
            .collect::<Vec<iced::Element<Message>>>();
        let back_button = button("Back").on_press(Message::GoToGames);
        column![back_button, title, Column::with_children(releases_list)].into()
    }
}
