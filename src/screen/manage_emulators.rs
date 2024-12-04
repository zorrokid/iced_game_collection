use crate::database::Database;
use crate::model::model::{Emulator, System};
use iced::widget::{button, checkbox, column, pick_list, row, text, text_input, Column};
use iced::Element;

pub struct ManageEmulators {
    pub emulator: Emulator,
    pub emulators: Vec<Emulator>,
    pub systems: Vec<System>,
}

#[derive(Debug, Clone)]
pub enum Message {
    NameChanged(String),
    SystemSelected(System),
    ExecutableChanged(String),
    ArgumentsChanged(String),
    Submit,
    GoHome,
    EditEmulator(String),
    DeleteEmulator(String),
    Clear,
    ExtractFilesChanged(bool),
    SupportedFileTypeExtensionsChanged(String),
}

pub enum Action {
    GoHome,
    None,
    EditEmulator(String),
    EmulatorSubmitted,
    EmulatorDeleted,
}

impl ManageEmulators {
    pub fn new(edit_emulator_id: Option<String>) -> Self {
        let db = Database::get_instance();
        let emulators = db.read().unwrap().get_emulators();
        let systems = db.read().unwrap().get_systems();
        let edit_emulator = edit_emulator_id.and_then(|id| db.read().unwrap().get_emulator(&id));
        Self {
            emulator: match edit_emulator {
                Some(emulator) => emulator,
                None => Emulator::default(),
            },
            emulators,
            systems,
        }
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
                let db = Database::get_instance();
                db.write()
                    .unwrap()
                    .add_or_update_emulator(self.emulator.clone());
                Action::EmulatorSubmitted
            }
            Message::SystemSelected(system) => {
                self.emulator.system_id = system.id;
                Action::None
            }
            Message::GoHome => Action::GoHome,
            Message::EditEmulator(id) => Action::EditEmulator(id),
            Message::DeleteEmulator(id) => {
                let db = Database::get_instance();
                db.write().unwrap().delete_emulator(&id);
                Action::EmulatorDeleted
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
                    button("Edit").on_press(Message::EditEmulator(emulator.id.clone())),
                    button("Delete").on_press(Message::DeleteEmulator(emulator.id.clone())),
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
