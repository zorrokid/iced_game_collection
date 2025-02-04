use iced::widget::{button, row, text_input};

use crate::{
    database_with_polo::DatabaseWithPolo,
    model::model::Franchise,
    repository::repository::FranchisReadRepository,
};

pub struct FranchiseSelect{
    new_franchise_name: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    FranchiseNameUpdated(String),
    Submit,
    CancelAddFranchise,
}

pub enum Action {
    FranchiseAdded(Franchise),
    CancelAddFranchise,
    None,
}

impl FranchiseSelect {
    pub fn new() -> Self {
        Self { new_franchise_name: "".to_string() }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Submit => {
               let db = DatabaseWithPolo::get_instance();
               if let Ok(id) = db.add_franchise(&self.new_franchise_name) {
                   Action::FranchiseAdded(Franchise { _id: Some(id), name: self.new_franchise_name.clone() })
                } else {
                    // TODO: show message to user
                    println!("Failed to add franchise");
                   Action::None
                }
            }
           Message::FranchiseNameUpdated(name) => {
                self.new_franchise_name = name;
                Action::None
            }
            Message::CancelAddFranchise => {
                Action::CancelAddFranchise
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        row![
            text_input("Franchise name", &self.new_franchise_name).on_input(Message::FranchiseNameUpdated),
            button("Submit franchise").on_press(Message::Submit),
            button("Cancel").on_press(Message::CancelAddFranchise) 
        ]
        .into()
    }
}
