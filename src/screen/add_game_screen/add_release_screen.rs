use std::path::PathBuf;

use crate::model::{Release, System};
use iced::widget::{button, column, pick_list, text, text_input, Column};
use iced::Task;

// TODO create of main and sub screens for add release
// - add release main
// -- add release screen
// -- add system screen
#[derive(Debug, Clone)]
pub struct AddReleaseScreen {
    pub name: String,
    pub systems: Vec<System>,
    pub selected_system: Option<System>,
    pub files: Vec<String>,
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

impl AddReleaseScreen {
    pub fn new(systems: Vec<System>) -> Self {
        Self {
            name: "".to_string(),
            systems,
            selected_system: None,
            files: vec![],
        }
    }

    pub fn title(&self) -> String {
        "Add Game".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Submit => {
                if let Some(system) = &self.selected_system {
                    Action::ReleaseAdded(Release {
                        id: 0, // TODO: maybe use Option, set id when saving to db?
                        name: self.name.clone(),
                        system: system.clone(),
                    })
                } else {
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
                println!("Select file");
                Action::Run(Task::perform(pick_file(), Message::FileAdded))
            }
            Message::FileAdded(result) => {
                println!("File added");
                if let Ok(path) = result {
                    self.files.push(path.to_string_lossy().to_string());
                }
                Action::None
            }
            Message::GoBack => Action::GoBack,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let title = text("Sub screen 2");
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
        column![
            back_button,
            title,
            release_name_input_field,
            systems_select,
            Column::with_children(files_list),
            add_file_button,
            submit_button
        ]
        .into()
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
}

async fn pick_file() -> Result<PathBuf, Error> {
    print!("pick file");
    let file_handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a text file")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;
    Ok(file_handle.path().to_owned())
}
