use std::path::PathBuf;

use crate::error::Error;
use crate::model::{Release, System};
use iced::widget::{button, column, pick_list, text, text_input, Column};
use iced::{Element, Task};

// TODO create of main and sub screens for add release
// - add release main
// -- add release screen
// -- add system screen
#[derive(Debug, Clone)]
pub struct ManageReleasesScreen {
    pub name: String,
    pub systems: Vec<System>,
    pub selected_system: Option<System>,
    pub files: Vec<String>,
    pub error: String,
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
}

pub enum Action {
    ReleaseAdded(Release),
    GoBack,
    None,
    Run(Task<Message>),
}

impl ManageReleasesScreen {
    pub fn new(systems: Vec<System>, releases: Vec<Release>) -> Self {
        Self {
            name: "".to_string(),
            systems,
            selected_system: None,
            files: vec![],
            error: "".to_string(),
            releases,
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Submit => {
                if let Some(system) = &self.selected_system {
                    Action::ReleaseAdded(Release {
                        id: 0, // TODO: maybe use Option, set id when saving to db?
                        name: self.name.clone(),
                        system: system.clone(),
                        files: self.files.clone(),
                    })
                } else {
                    self.error = "System not selected".to_string();
                    Action::None
                }
            }
            Message::NameChanged(name) => {
                self.name = name.clone();
                Action::None
            }
            Message::SystemSelected(system) => {
                self.selected_system = Some(system);
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
                    self.files.push(path.to_string_lossy().to_string());
                }
                Action::None
            }
            Message::GoBack => Action::GoBack,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let releases_label = text("Releases");
        let releases_list = self
            .releases
            .iter()
            .map(|release| text(release.to_string()).into())
            .collect::<Vec<Element<Message>>>();

        let release_name_input_field =
            text_input("Enter release name", &self.name).on_input(Message::NameChanged);
        let systems_select = pick_list(
            self.systems.as_slice(),
            self.selected_system.as_ref(),
            Message::SystemSelected,
        );
        let files_list = self
            .files
            .iter()
            .map(|file| text(file).into())
            .collect::<Vec<iced::Element<Message>>>();
        let add_file_button = button("Add File").on_press(Message::SelectFile);
        let back_button = button("Back").on_press(Message::GoBack);

        let submit_button = button("Submit").on_press(Message::Submit);
        let error = text(self.error.clone());
        column![
            back_button,
            releases_label,
            Column::with_children(releases_list),
            release_name_input_field,
            systems_select,
            Column::with_children(files_list),
            add_file_button,
            submit_button,
            error,
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
