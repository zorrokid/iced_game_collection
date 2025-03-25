use std::{cell::OnceCell, sync::Arc};

use crate::{
    database::{
        database_error::DatabaseError, repository_manager::RepositoryManager,
        software_title_repository::SoftwareTitleReadRepository,
    },
    error::Error,
    model::model::SoftwareTitle,
    service::view_model_service::ViewModelService,
    view_model::list_models::ReleaseListModel,
};
use iced::{
    widget::{button, column, row, text, Column},
    Length, Task,
};

// TODO: ViewGame needs to be a main screen with subscreens:
// - view game main screen with list of releases
// - view release screen
// - edit release screen
// - view image screen
#[derive(Debug, Clone)]
pub struct ViewGame {
    repo: Arc<RepositoryManager>,
    view_model_service: Arc<ViewModelService>,
    game: OnceCell<SoftwareTitle>,
    releases: Vec<ReleaseListModel>,
}

#[derive(Debug, Clone)]
pub enum Message {
    GoToGames,
    EditRelease(i64),
    ViewRelease(i64),
    DeleteRelease(i64),
    ReleasesLoaded(Result<Vec<ReleaseListModel>, DatabaseError>),
    GameLoaded(Result<SoftwareTitle, DatabaseError>),
}

#[derive(Debug, Clone)]
pub enum Action {
    Back,
    EditRelease(i64),
    ViewRelease(i64),
    None,
    Error(Error),
}

impl ViewGame {
    pub fn new(
        repo: Arc<RepositoryManager>,
        view_model_service: Arc<ViewModelService>,
        game_id: i64,
    ) -> (Self, Task<Message>) {
        let service_clone = Arc::clone(&view_model_service);
        let load_releases_task = Task::perform(
            async move { service_clone.get_release_list_models(game_id).await },
            Message::ReleasesLoaded,
        );

        let repo_clone = Arc::clone(&repo);
        let load_game_task = Task::perform(
            async move {
                repo_clone
                    .software_titles()
                    .get_software_title(game_id)
                    .await
            },
            Message::GameLoaded,
        );

        let combined_task = Task::batch(vec![load_releases_task, load_game_task]);

        (
            Self {
                repo,
                view_model_service,
                game: OnceCell::new(),
                releases: vec![],
            },
            combined_task,
        )
    }

    pub fn title(&self) -> String {
        format!("{} releases", self.game.name)
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::GoToGames => Action::Back,
            Message::EditRelease(id) => Action::EditRelease(id),
            Message::ViewRelease(id) => Action::ViewRelease(id),
            Message::DeleteRelease(id) => {
                let db = DatabaseWithPolo::get_instance();
                match db.delete_release(&id) {
                    Ok(_) => {
                        self.releases.retain(|release| release.id != id);
                        Action::None
                    }
                    Err(e) => Action::Error(e),
                }
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let title = text(self.game.name.clone()).size(30);

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
                            .then_some(Message::DeleteRelease(release.id)),
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
        let back_button = button("Back").on_press(Message::GoToGames);
        column![back_button, title, Column::with_children(releases_list)].into()
    }
}
