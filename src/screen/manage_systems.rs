use crate::error::Error;
use crate::files::pick_folder;
use crate::model::{get_new_id, FolderType, System};
use iced::widget::{button, column, row, text, text_input, Column};
use iced::Task;
use std::path::PathBuf;

// TODO: move AddSystem under Add Release?
pub struct ManageSystems {
    pub system: System,
    pub systems: Vec<System>,
}

#[derive(Debug, Clone)]
pub enum Message {
    NameChanged(String),
    GoHome,
    Submit,
    EditSystem(i32),
    DeleteSystem(i32),
    SelectFolder(FolderType),
    FolderAdded(Result<(PathBuf, FolderType), Error>),
}

pub enum Action {
    SubmitSystem(System),
    GoHome,
    None,
    EditSystem(i32),
    DeleteSystem(i32),
    Run(Task<Message>),
}

impl ManageSystems {
    pub fn new(systems: Vec<System>, edit_system: Option<System>) -> Self {
        Self {
            system: match edit_system {
                Some(system) => system,
                None => System {
                    id: get_new_id(&systems),
                    name: "".to_string(),
                    roms_source_path: "".to_string(),
                    roms_destination_path: "".to_string(),
                },
            },
            systems,
        }
    }

    pub fn title(&self) -> String {
        "Manage systems".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::NameChanged(name) => {
                self.system.name = name;
                Action::None
            }
            Message::Submit => {
                if self.system.name.is_empty() {
                    return Action::None;
                } else {
                    Action::SubmitSystem(self.system.clone())
                }
            }
            Message::GoHome => Action::GoHome,
            Message::EditSystem(id) => Action::EditSystem(id),
            Message::DeleteSystem(id) => Action::DeleteSystem(id),
            Message::SelectFolder(folder_type) => Action::Run(Task::perform(
                pick_folder(folder_type),
                Message::FolderAdded,
            )),
            Message::FolderAdded(Ok((path, folder_type))) => {
                match folder_type {
                    FolderType::Source => {
                        self.system.roms_source_path = path.to_string_lossy().to_string()
                    }
                    FolderType::Destination => {
                        self.system.roms_destination_path = path.to_string_lossy().to_string()
                    }
                }
                Action::None
            }
            Message::FolderAdded(Err(err)) => {
                print!("Error adding folder: {:?}", err);
                Action::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let name_input_field =
            text_input("Enter name", &self.system.name).on_input(Message::NameChanged);
        let add_button = button("Submit").on_press(Message::Submit);
        let systems_list = self
            .systems
            .iter()
            .map(|system| {
                row![
                    text(system.to_string()).width(iced::Length::Fixed(300.0)),
                    button("Edit").on_press(Message::EditSystem(system.id)),
                    button("Delete").on_press(Message::DeleteSystem(system.id)),
                ]
                .into()
            })
            .collect::<Vec<iced::Element<Message>>>();

        let folders_title = text("Select source and destination folders for roms/software images");
        let add_source_folder_button =
            button("Select source folder").on_press(Message::SelectFolder(FolderType::Source));

        let source_folder_text = text(format!("Source: {}", self.system.roms_source_path));

        let add_destination_folder_button = button("Select destination folder")
            .on_press(Message::SelectFolder(FolderType::Destination));

        let destination_folder_text = text(format!(
            "Destination: {}",
            self.system.roms_destination_path
        ));

        let back_button = button("Back").on_press(Message::GoHome);
        column![
            back_button,
            name_input_field,
            folders_title,
            add_source_folder_button,
            source_folder_text,
            add_destination_folder_button,
            destination_folder_text,
            add_button,
            Column::with_children(systems_list)
        ]
        .into()
    }
}
