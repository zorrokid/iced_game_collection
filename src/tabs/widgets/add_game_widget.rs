use iced::widget::{button, column, pick_list, row, text_input};

use crate::{
    database_with_polo::DatabaseWithPolo,
    model::model::Franchise,
    repository::repository::FranchisReadRepository,
};

use super::franchise_widget::{self, FranchiseSelect};

pub struct AddGame {
    franchises: Vec<Franchise>,
    selected_franchise: Option<Franchise>,
    game_name: String,
    francise_select: FranchiseSelect,
    adding_franchise: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    FranchiseSelected(Franchise),
    Submit,
    SetAddFranchise,
    AddFranchise(franchise_widget::Message),
}

pub enum Action {
    GameAdded,
    None,
}

impl AddGame {
    pub fn new() -> Self {
       let db = DatabaseWithPolo::get_instance();
       let franchises = db.get_all_franchises().unwrap_or_else(|err| {
            println!("Failed to get franchises {:?}", err);
            vec![]
        });
        Self { franchises, selected_franchise: None, game_name: "".to_string(), francise_select: FranchiseSelect::new(), adding_franchise: false }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::FranchiseSelected(franchise) => {
                self.selected_franchise = Some(franchise);
                Action::None
            }
            Message::Submit => {
                Action::GameAdded
            }
            Message::SetAddFranchise => {
                self.adding_franchise = true;
                Action::None
            }
            Message::AddFranchise(message) => {
                let action = self.francise_select.update(message);
                if let franchise_widget::Action::FranchiseAdded(franchise) = action {
                    self.franchises.push(franchise.clone());
                    self.adding_franchise = false;
                }
                Action::None
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let franchise_row = row![
            self.create_franchise_dropdown(),
            button("Add Franchise").on_press(Message::SetAddFranchise)
        ];
        let add_franchise_row = if self.adding_franchise {
            self.francise_select.view().map(Message::AddFranchise)
        } else {
            row![].into()
        };
        
        column![
            text_input("Game Name", &self.game_name),
            franchise_row,
            add_franchise_row,
            button("Submit")
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
