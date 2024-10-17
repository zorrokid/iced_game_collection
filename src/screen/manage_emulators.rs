use crate::model::{get_new_id, Emulator, System};
use iced::widget::{button, column, pick_list, row, text, text_input, Column};
use iced::Element;

pub struct ManageEmulators {
    pub emulator: Emulator,
    pub emulators: Vec<Emulator>,
    pub systems: Vec<System>,
    pub selected_system: Option<System>,
}

#[derive(Debug, Clone)]
pub enum Message {
    NameChanged(String),
    SystemSelected(System),
    ExecutableChanged(String),
    ArgumentsChanged(String),
    Submit(Emulator),
    GoHome,
    EditEmulator(i32),
    DeleteEmulator(i32),
}

pub enum Action {
    SubmitEmulator(Emulator),
    GoHome,
    None,
    EditEmulator(i32),
    DeleteEmulator(i32),
}

impl ManageEmulators {
    pub fn new(
        emulators: Vec<Emulator>,
        systems: Vec<System>,
        edit_emulator: Option<Emulator>,
    ) -> Self {
        Self {
            emulator: match edit_emulator {
                Some(emulator) => emulator,
                None => Emulator {
                    id: get_new_id(&emulators),
                    name: "".to_string(),
                    executable: "".to_string(),
                    arguments: "".to_string(),
                    system_id: 0,
                },
            },
            emulators,
            systems,
            selected_system: None,
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
            Message::Submit(emulator) => {
                if self.emulator.name.is_empty() || self.emulator.executable.is_empty() {
                    return Action::None;
                }
                Action::SubmitEmulator(emulator)
            }
            Message::SystemSelected(system) => {
                self.emulator.system_id = system.id;
                self.selected_system = Some(system);
                Action::None
            }
            Message::GoHome => Action::GoHome,
            Message::EditEmulator(id) => Action::EditEmulator(id),
            Message::DeleteEmulator(id) => Action::DeleteEmulator(id),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let name_input_field =
            text_input("Enter name", &self.emulator.name).on_input(Message::NameChanged);
        let executable_input_field = text_input("Enter executable", &self.emulator.executable)
            .on_input(Message::ExecutableChanged);
        let systems_select = pick_list(
            self.systems.as_slice(),
            self.selected_system.as_ref(),
            Message::SystemSelected,
        );
        let arguments_input_field = text_input("Enter arguments", &self.emulator.arguments)
            .on_input(Message::ArgumentsChanged);
        let add_button = button("Submit").on_press(Message::Submit(self.emulator.clone()));
        let emulators_list = self
            .emulators
            .iter()
            .map(|emulator| {
                row![
                    text(emulator.name.to_string()),
                    button("Edit").on_press(Message::EditEmulator(emulator.id)),
                    button("Delete").on_press(Message::DeleteEmulator(emulator.id)),
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
            systems_select,
            add_button,
            Column::with_children(emulators_list)
        ]
        .into()
    }
}
