use std::path::PathBuf;

use crate::error::Error;
use crate::model::{get_new_id, Release, System};
use iced::widget::{button, column, pick_list, row, text, text_input, Column};
use iced::{Element, Task};

// TODO create of main and sub screens for add release
// - add release main
// -- add release screen
// -- add system screen
#[derive(Debug, Clone)]
pub struct ManageReleasesScreen {
    pub release: Release,
    pub systems: Vec<System>,
    pub releases: Vec<Release>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Submit,
    NameChanged(String),
    SystemSelected(System),
    GoBack,
    FileAdded(Result<PathBuf, Error>),
    SelectFile,
    Edit(i32),
    Delete(i32),
}

pub enum Action {
    SubmitRelease(Release),
    GoBack,
    None,
    Run(Task<Message>),
    Edit(i32),
    Delete(i32),
}

impl ManageReleasesScreen {
    pub fn new(
        systems: Vec<System>,
        releases: Vec<Release>,
        edit_release: Option<Release>,
    ) -> Self {
        Self {
            release: match edit_release {
                Some(release) => release,
                None => Release {
                    id: get_new_id(&releases),
                    name: "".to_string(),
                    system_id: 0,
                    files: vec![],
                },
            },
            systems,
            releases,
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Submit => match (self.release.system_id, self.release.name.is_empty()) {
                (0, _) => Action::None,
                (_, true) => Action::None,
                _ => Action::SubmitRelease(self.release.clone()),
            },
            Message::NameChanged(name) => {
                self.release.name = name.clone();
                Action::None
            }
            Message::SystemSelected(system) => {
                self.release.system_id = system.id;
                Action::None
            }
            Message::SelectFile => {
                // Why do we need to wrap the Task in Run-action?
                // - since update returns an Action, we need to wrap the Task in an Action
                // - also the Task::perform needs to be returned to the runtime
                Action::Run(
                    // Async operation pick_file has to be run in a separate thread
                    // the outcome of pick_file is sent back to the main thread as a FileAdded-Message
                    Task::perform(pick_file(), Message::FileAdded),
                )
            }
            Message::FileAdded(result) => {
                if let Ok(path) = result {
                    self.release.files.push(path.to_string_lossy().to_string());
                }
                Action::None
            }
            Message::GoBack => Action::GoBack,
            Message::Delete(id) => Action::Delete(id),
            Message::Edit(id) => Action::Edit(id),
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let releases_label = text("Releases");
        let releases_list = self
            .releases
            .iter()
            .map(|release| {
                row![
                    text(release.to_string()),
                    button("Edit").on_press(Message::Edit(release.id)),
                    button("Delete").on_press(Message::Delete(release.id))
                ]
                .into()
            })
            .collect::<Vec<Element<Message>>>();

        let release_name_input_field =
            text_input("Enter release name", &self.release.name).on_input(Message::NameChanged);

        let selected_system = self
            .systems
            .iter()
            .find(|system| system.id == self.release.system_id);

        let systems_select = pick_list(
            self.systems.as_slice(),
            selected_system,
            Message::SystemSelected,
        );
        let files_list = self
            .release
            .files
            .iter()
            .map(|file| text(file).into())
            .collect::<Vec<iced::Element<Message>>>();
        let add_file_button = button("Add File").on_press(Message::SelectFile);
        let back_button = button("Back").on_press(Message::GoBack);

        let submit_button = button("Submit").on_press(Message::Submit);
        column![
            back_button,
            releases_label,
            Column::with_children(releases_list),
            release_name_input_field,
            systems_select,
            Column::with_children(files_list),
            add_file_button,
            submit_button,
        ]
        .into()
    }
}

async fn pick_file() -> Result<PathBuf, Error> {
    let file_handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a file")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;
    Ok(file_handle.path().to_owned())
}
