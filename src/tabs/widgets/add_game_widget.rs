use iced::widget::{button, column, pick_list, row, text_input};

use crate::{
    database_with_polo::DatabaseWithPolo,
    model::model::{Franchise, Game, WithId},
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
    GameNameUpdated(String),
    CancelAddGame,
}

pub enum Action {
    GameAdded(Game),
    CancelAddGame,
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
                let db = DatabaseWithPolo::get_instance();
                let franchise_id = if let Some(franchise) = self.selected_franchise.as_ref() {
                    franchise._id.clone()
                } else {
                    None
                };
                let game_to_db = Game { _id: None, name: self.game_name.clone(), franchise_id };
                if let Ok(id) = db.add_game(&game_to_db) {
                   Action::GameAdded(game_to_db.clone().with_id(id))
                } else {
                    Action::None
                }
            }
            Message::SetAddFranchise => {
                self.adding_franchise = true;
                Action::None
            }
            Message::AddFranchise(message) => {
                let action = self.francise_select.update(message);
                match action {
                    franchise_widget::Action::FranchiseAdded(franchise) => {
                        self.franchises.push(franchise.clone());
                        self.adding_franchise = false;
                    }
                    franchise_widget::Action::CancelAddFranchise => {
                        self.adding_franchise = false;
                    }
                    _ => {}
                }
                Action::None
            }
            Message::GameNameUpdated(name) => {
                self.game_name = name;
                Action::None
            }
            Message::CancelAddGame => {
                self.adding_franchise = false;
                Action::CancelAddGame
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let franchise_row = row![
            self.create_franchise_dropdown(),
            button("Add Franchise").on_press_maybe((!self.adding_franchise).then(|| Message::SetAddFranchise))
        ];
        let add_franchise_row = if self.adding_franchise {
            self.francise_select.view().map(Message::AddFranchise)
        } else {
            row![].into()
        };

        let add_game_row = row![
            text_input("Game Name", &self.game_name).on_input(Message::GameNameUpdated),
            button("Submit game").on_press_maybe((!self.game_name.is_empty()).then(|| Message::Submit)),
            button("Cancel").on_press(Message::CancelAddGame),
        ];
        
        column![
            add_game_row,
            franchise_row,
            add_franchise_row,
        ]
        .into()
    }

    fn create_franchise_dropdown(&self) -> iced::Element<Message> {
        pick_list(
            self.franchises.clone(),
            self.selected_franchise.clone(),
            Message::FranchiseSelected,
        ).into()
    }
}
