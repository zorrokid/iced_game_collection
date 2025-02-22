use iced::widget::{button, row, text_input};

use crate::{
    database_with_polo::DatabaseWithPolo,
    model::model::{HasOid, System},
    repository::repository::SystemWriteRepository,
};

pub struct AddSystem {
    new_system_name: String,
    system_notes: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    SystemNameUpdated(String),
    Submit,
    CancelAddSystem,
    NotesChanged(String),
}

pub enum Action {
    SystemAdded(System),
    CancelAddSystem,
    None,
}

impl AddSystem {
    pub fn new() -> Self {
        Self {
            new_system_name: "".to_string(),
            system_notes: "".to_string(),
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Submit => {
                let db = DatabaseWithPolo::get_instance();

                let system_to_db = System {
                    _id: None,
                    name: self.new_system_name.clone(),
                    notes: if self.system_notes.is_empty() {
                        None
                    } else {
                        Some(self.system_notes.clone())
                    },
                };
                if let Ok(id) = db.add_system(&system_to_db) {
                    Action::SystemAdded(system_to_db.clone().with_id(id))
                } else {
                    // TODO: show message to user
                    println!("Failed to add system");
                    Action::None
                }
            }
            Message::SystemNameUpdated(name) => {
                self.new_system_name = name;
                Action::None
            }
            Message::CancelAddSystem => Action::CancelAddSystem,
            Message::NotesChanged(notes) => {
                self.system_notes = notes;
                Action::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let name_input =
            text_input("System name", &self.new_system_name).on_input(Message::SystemNameUpdated);
        let notes_field =
            text_input("Enter notes", &self.system_notes).on_input(Message::NotesChanged);

        let submit_button = button("Submit system")
            .on_press_maybe((!self.new_system_name.is_empty()).then_some(Message::Submit));
        let cancel_button = button("Cancel").on_press(Message::CancelAddSystem);
        row![name_input, notes_field, submit_button, cancel_button].into()
    }
}
