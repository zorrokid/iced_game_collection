use crate::error::Error;
use crate::files::{delete_files, pick_file};
use crate::model::{get_new_id, Release, System};
use async_std::path::PathBuf;
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
    FilesDeleted(Result<i32, Error>),
}

pub enum Action {
    SubmitRelease(Release),
    GoBack,
    None,
    Run(Task<Message>),
    Edit(i32),
    Delete(i32),
    Error(String),
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
                    games: vec![],
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
                let selected_system = self
                    .systems
                    .iter()
                    .find(|system| system.id == self.release.system_id);
                if let Some(system) = selected_system {
                    let source_path = system.roms_source_path.clone();
                    let destination_path = system.roms_destination_path.clone();
                    // We need to wrap the Task in an Action, because with Action we can pass the Task back to the main update-function which
                    // returns a Task<Message> which is then passed back to the iced runtime. Iced runtime passes the Message with the result from the
                    // Task back to the update function.
                    Action::Run(Task::perform(
                        pick_file(source_path, destination_path),
                        Message::FileAdded,
                    ))
                } else {
                    Action::None
                }
            }
            Message::FileAdded(result) => {
                if let Ok(path) = result {
                    if let Some(file_name) = path
                        .file_name()
                        .and_then(|os_str| os_str.to_str().map(|s| s.to_string()))
                    {
                        self.release.files.push(file_name);
                    }
                }
                Action::None
            }
            Message::GoBack => Action::GoBack,
            Message::Delete(id) => {
                match self
                    .releases
                    .iter()
                    .find(|release| release.id == id)
                    .and_then(|release| {
                        self.systems
                            .iter()
                            .find(|system| system.id == release.system_id)
                            .map(|system| {
                                Action::Run(Task::perform(
                                    delete_files(
                                        release.files.clone(),
                                        system.roms_destination_path.clone(),
                                        id,
                                    ),
                                    Message::FilesDeleted,
                                ))
                            })
                    }) {
                    Some(action) => action,
                    None => Action::None,
                }
            }
            Message::Edit(id) => Action::Edit(id),
            Message::FilesDeleted(result) => match result {
                Ok(id) => Action::Delete(id),
                Err(e) => Action::Error(format!("Failed to delete files: {}", e)),
            },
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let releases_label = text("Releases");
        let releases_list = self
            .releases
            .iter()
            .map(|release| {
                row![
                    text(release.to_string()).width(iced::Length::Fixed(300.0)),
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
