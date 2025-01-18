use bson::oid::ObjectId;
use iced::{widget::text, Task};

pub struct ReleaseDetails {
    game_id: Option<ObjectId>, // Add fields here
}

#[derive(Debug, Clone)]
pub enum Message {
    ReleaseSelected(ObjectId),
    GameSelected(ObjectId),
    // Add message variants here
}

impl ReleaseDetails {
    pub fn new() -> Self {
        // TODO: load releases for game_id if set
        Self { game_id: None }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        // TODO: handle ReleaseSelected message
        // Update fields here
        Task::none()
    }

    pub fn view(&self) -> iced::Element<Message> {
        // TODO: display list of releases, selection send ReleaseSelected message
        text("Release details").into()
    }
}
