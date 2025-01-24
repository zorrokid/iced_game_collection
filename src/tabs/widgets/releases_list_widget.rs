use bson::oid::ObjectId;
use iced::{
    widget::{button, row, text, Column},
    Length,
};

use crate::{
    database_with_polo::DatabaseWithPolo,
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
    EditRelease(ObjectId),
    DeleteRelease(ObjectId),
}

pub enum Action {
    EditRelease(ObjectId),
    None,
}

impl ReleasesList {
    pub fn new() -> Self {
        Self {
            game: None,
            releases: vec![],
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
            Message::DeleteRelease(id) => {
                match DatabaseWithPolo::get_instance().delete_release(&id) {
                    Ok(_) => {
                        self.releases.retain(|release| release.id != id);
                        Action::None
                    }
                    Err(e) => {
                        // TODO: Show error message
                        println!("Failed to delete release {:?}", e);
                        Action::None
                    }
                }
            }
            Message::EditRelease(id) => Action::EditRelease(id),
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let releases_list = self
            .releases
            .iter()
            .map(|release| {
                let edit_release_button = button("Edit")
                    .on_press(Message::EditRelease(release.id))
                    .width(Length::Fixed(100.0));
                let view_release_button = button("View")
                    .on_press(Message::ViewRelease(release.id))
                    .width(Length::Fixed(100.0));
                let delete_button = button("Delete")
                    .on_press_maybe(
                        release
                            .can_delete
                            .then(|| Message::DeleteRelease(release.id)),
                    )
                    .width(Length::Fixed(100.0));

                let release_row = row![
                    text(&release.name).width(Length::Fixed(100.0)),
                    text(&release.system_name).width(Length::Fixed(100.0)),
                    view_release_button,
                    edit_release_button,
                    delete_button,
                ];

                release_row.into()
            })
            .collect::<Vec<iced::Element<Message>>>();
        Column::with_children(releases_list).into()
    }
}
