use iced::widget::{button, column, pick_list, text_input};

use crate::{
    database_with_polo::DatabaseWithPolo,
    model::model::Franchise,
    repository::repository::FranchisReadRepository,
};

pub struct FranchiseSelect{
    // TODO: I wonder if this struct should be only for adding new franchises, and selecting
    // francise for game should be in add_game_widget instead
    franchises: Vec<Franchise>,
    selected_franchise: Option<Franchise>,
    new_franchise_name: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    FranchiseSelected(Franchise),
    FranchiseAdded,
    FranchiseNameUpdated(String),
    Submit,
}

pub enum Action {
    FranchiseSelected(Franchise),
    FranchiseAdded(Franchise),
    None,
}

impl FranchiseSelect {
    pub fn new() -> Self {
       let db = DatabaseWithPolo::get_instance();
       let franchises = db.get_all_franchises().unwrap_or_else(|err| {
            println!("Failed to get franchises {:?}", err);
            vec![]
        });
        Self { franchises, selected_franchise: None, new_franchise_name: "".to_string() }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::FranchiseSelected(franchise) => {
                self.selected_franchise = Some(franchise.clone());
                Action::FranchiseSelected(franchise)
            }
            Message::Submit => {
               let db = DatabaseWithPolo::get_instance();
               if let Ok(id) = db.add_franchise(&self.new_franchise_name) {
                   self.franchises.push(Franchise { _id: Some(id), name: self.new_franchise_name.clone() });
                   // TODO: sort franchises
                   Action::FranchiseAdded(Franchise { _id: Some(id), name: self.new_franchise_name.clone() })
                } else {
                    // TODO: show message to user
                    println!("Failed to add franchise");
                   Action::None
                }
            }
            Message::FranchiseAdded => {
                Action::None
            }
            Message::FranchiseNameUpdated(name) => {
                self.new_franchise_name = name;
                Action::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        column![
            self.create_franchise_dropdown(),
            text_input("Franchise name", &self.new_franchise_name).on_input(Message::FranchiseNameUpdated),
            button("Submit").on_press(Message::Submit)
        ]
        .into()
    }

    fn create_franchise_dropdown(&self) -> iced::Element<Message> {
        let pick_list = pick_list(
            self.franchises.clone(),
            self.selected_franchise.clone(),
            Message::FranchiseSelected,
        );
        pick_list.into()
    }
}
