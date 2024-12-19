use crate::database_with_polo::DatabaseWithPolo;
use crate::error::Error;
use crate::model::model::{Emulator, HasOid, System};
use iced::widget::{button, checkbox, column, pick_list, row, text, text_input, Column};
use iced::Element;
use polodb_core::bson::oid::ObjectId;

pub struct ManageEmulators {
    pub emulator: Emulator,
    pub emulators: Vec<Emulator>,
    pub systems: Vec<System>,
    pub is_edit: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    NameChanged(String),
    SystemSelected(System),
    ExecutableChanged(String),
    ArgumentsChanged(String),
    Submit,
    GoHome,
    EditEmulator(ObjectId),
    DeleteEmulator(ObjectId),
    Clear,
    ExtractFilesChanged(bool),
    SupportedFileTypeExtensionsChanged(String),
}

pub enum Action {
    GoHome,
    None,
    EditEmulator(ObjectId),
    EmulatorSubmitted,
    EmulatorDeleted,
    Error(Error),
}

impl ManageEmulators {
    pub fn new(edit_emulator_id: Option<ObjectId>) -> Result<Self, Error> {
        let db = DatabaseWithPolo::get_instance();
        let emulators = db.get_emulators()?;
        let systems = db.get_systems()?;
        let is_edit = edit_emulator_id.is_some();

        let edit_emulator = match edit_emulator_id {
            Some(id) => db.get_emulator(&id),
            None => Ok(None),
        }?;

        Ok(Self {
            emulator: match edit_emulator {
                Some(emulator) => emulator,
                None => Emulator::default(),
            },
            emulators,
            systems,
            is_edit,
        })
    }

    pub fn title(&self) -> String {
        "Manage emulators".to_string()
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::NameChanged(name) => {
                self.emulator.name = name;
                Action::None
            }
            Message::ExecutableChanged(executable) => {
                self.emulator.executable = executable;
                Action::None
            }
            Message::ArgumentsChanged(arguments) => {
                self.emulator.arguments = arguments;
                Action::None
            }
            Message::Submit => {
                if self.emulator.name.is_empty() || self.emulator.executable.is_empty() {
                    return Action::None;
                }

                let db_new = DatabaseWithPolo::get_instance();
                match self.is_edit {
                    true => db_new.update_emulator(&self.emulator),
                    false => db_new.add_emulator(&self.emulator),
                };
                Action::EmulatorSubmitted
            }
            Message::SystemSelected(system) => {
                self.emulator.system_id = system.id;
                Action::None
            }
            Message::GoHome => Action::GoHome,
            Message::EditEmulator(id) => Action::EditEmulator(id),
            Message::DeleteEmulator(id) => {
                let db = DatabaseWithPolo::get_instance();
                match db.delete_emulator(&id) {
                    Ok(_) => Action::EmulatorDeleted,
                    Err(e) => Action::Error(e),
                }
            }
            Message::Clear => {
                self.emulator = Emulator::default();
                Action::None
            }
            Message::ExtractFilesChanged(is_checked) => {
                self.emulator.extract_files = is_checked;
                Action::None
            }
            Message::SupportedFileTypeExtensionsChanged(extensions) => {
                self.emulator.supported_file_type_extensions = extensions
                    .split(',')
                    .map(|s| s.trim().to_string().to_lowercase())
                    .collect();
                Action::None
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let name_input_field =
            text_input("Enter name", &self.emulator.name).on_input(Message::NameChanged);
        let executable_input_field = text_input("Enter executable", &self.emulator.executable)
            .on_input(Message::ExecutableChanged);
        let systems_select = pick_list(
            self.systems.as_slice(),
            self.systems
                .iter()
                .find(|s| s.id == self.emulator.system_id),
            Message::SystemSelected,
        );
        let arguments_input_field = text_input("Enter arguments", &self.emulator.arguments)
            .on_input(Message::ArgumentsChanged);
        let extract_files_checkbox = checkbox("Extract files", self.emulator.extract_files)
            .on_toggle(Message::ExtractFilesChanged);
        let main_buttons = row![
            button("Submit").on_press(Message::Submit),
            button("Clear").on_press(Message::Clear)
        ];

        let supported_file_type_extensions = text_input(
            "Enter supported file type extensions (as comma separated list)",
            &self.emulator.supported_file_type_extensions.join(", "),
        )
        .on_input(Message::SupportedFileTypeExtensionsChanged);

        let emulators_list = self
            .emulators
            .iter()
            .map(|emulator| {
                row![
                    text(emulator.name.to_string()).width(iced::Length::Fixed(300.0)),
                    button("Edit").on_press(Message::EditEmulator(emulator.id())),
                    button("Delete").on_press(Message::DeleteEmulator(emulator.id())),
                ]
                .into()
            })
            .collect::<Vec<Element<Message>>>();
        let back_button = button("Back").on_press(Message::GoHome);
        column![
            back_button,
            name_input_field,
            executable_input_field,
            arguments_input_field,
            supported_file_type_extensions,
            systems_select,
            extract_files_checkbox,
            main_buttons,
            Column::with_children(emulators_list)
        ]
        .into()
    }
}
