use std::path::PathBuf;

use iced::{widget::image, Element};

pub struct ImageViewer {
    pub image_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ImageSelected(PathBuf),
    Clear,
}

impl ImageViewer {
    pub fn new() -> Self {
        Self { image_path: None }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ImageSelected(path) => {
                self.image_path = Some(path);
            }
            Message::Clear => {
                self.image_path = None;
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        if let Some(path) = &self.image_path {
            let image = image(path.clone());
            image.into()
        } else {
            "No image selected".into()
        }
    }
}
