use iced::widget::{button, column, row, text_input};
#[derive(Debug, Clone)]
pub struct SettingsMainScreen {
    collection_root_dir: String,
    is_locked: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    SetCollectionRootDir(String),
    Submit,
    Back,
}

pub enum Action {
    Back,
    SetCollectionRootDir(String),
    None,
}

impl SettingsMainScreen {
    pub fn new(collection_root_dir: String) -> Self {
        Self {
            is_locked: !collection_root_dir.clone().is_empty(),
            collection_root_dir,
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::SetCollectionRootDir(collection_root_dir) => match self.is_locked {
                true => Action::None,
                false => {
                    self.collection_root_dir = collection_root_dir.clone();
                    Action::None
                }
            },
            Message::Submit => {
                self.is_locked = true;
                Action::SetCollectionRootDir(self.collection_root_dir.clone())
            }
            Message::Back => Action::Back,
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        let collection_root_dir_input =
            text_input("Collection root dir", &self.collection_root_dir)
                .on_input(Message::SetCollectionRootDir);
        let save_button = button("Submit").on_press(Message::Submit);
        let root_dir_row = row![collection_root_dir_input, save_button];

        let back_button = button("Back").on_press(Message::Back);

        column![back_button, root_dir_row].into()
    }
}
