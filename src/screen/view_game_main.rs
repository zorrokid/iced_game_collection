use bson::oid::ObjectId;

use crate::{emulator_runner::EmulatorRunOptions, error::Error};

use super::{
    add_release_main, view_game, view_game_screen::ViewGameScreen, view_image, view_release,
};

#[derive(Debug, Clone)]
pub struct ViewGameMain {
    screen: ViewGameScreen,
    game_id: ObjectId,
}

#[derive(Debug, Clone)]
pub enum Message {
    ViewGameScreen(view_game::Message),
    ViewImageScreen(view_image::Message),
    ViewReleaseScreen(view_release::Message),
    EditReleaseScreen(add_release_main::Message),
}

pub enum Action {
    Back,
    None,
    Run(iced::Task<Message>),
    Error(Error),
    RunWithEmulator(EmulatorRunOptions),
}

impl ViewGameMain {
    pub fn new(game_id: ObjectId) -> Result<Self, Error> {
        let screen = view_game::ViewGame::new(game_id)?;

        Ok(Self {
            screen: ViewGameScreen::ViewGame(screen),
            game_id,
        })
    }

    pub fn title(&self) -> String {
        match &self.screen {
            ViewGameScreen::ViewGame(screen) => screen.title(),
            // TODO should this be in ViewReleaseMain?
            ViewGameScreen::ViewImage(screen) => screen.title(),
            ViewGameScreen::ViewRelease(screen) => screen.title(),
            ViewGameScreen::EditRelease(screen) => screen.title(),
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ViewGameScreen(message) => match &mut self.screen {
                ViewGameScreen::ViewGame(screen) => {
                    let action = screen.update(message);
                    match action {
                        view_game::Action::EditRelease(id) => {
                            match add_release_main::AddReleaseMain::new(Some(id)) {
                                Ok(add_release_main) => {
                                    self.screen = ViewGameScreen::EditRelease(add_release_main);
                                    Action::None
                                }
                                Err(e) => Action::Error(e),
                            }
                        }
                        view_game::Action::ViewRelease(id) => {
                            match view_release::ViewRelease::new(id) {
                                Ok(view_release) => {
                                    self.screen = ViewGameScreen::ViewRelease(view_release);
                                    Action::None
                                }
                                Err(e) => Action::Error(e),
                            }
                        }
                        view_game::Action::Back => Action::Back,
                        view_game::Action::Error(e) => Action::Error(e),
                        view_game::Action::None => Action::None,
                    }
                }
                _ => Action::None,
            },
            Message::ViewImageScreen(message) => match &mut self.screen {
                ViewGameScreen::ViewImage(screen) => {
                    let action = screen.update(message);
                    match action {
                        view_image::Action::Back => Action::Back,
                    }
                }
                _ => Action::None,
            },
            Message::ViewReleaseScreen(message) => match &mut self.screen {
                ViewGameScreen::ViewRelease(screen) => match screen.update(message) {
                    view_release::Action::Back => self.create_main_screen(),
                    view_release::Action::RunWithEmulator(options) => {
                        Action::RunWithEmulator(options)
                    }
                    view_release::Action::ViewImage(file_path) => {
                        let screen = view_image::ViewImage::new(file_path);
                        self.screen = ViewGameScreen::ViewImage(screen);
                        Action::None
                    }
                    view_release::Action::None => Action::None,
                    view_release::Action::Run(task) => {
                        Action::Run(task.map(Message::ViewReleaseScreen))
                    }
                    view_release::Action::Error(error) => Action::Error(error),
                },
                _ => Action::None,
            },
            Message::EditReleaseScreen(message) => match &mut self.screen {
                ViewGameScreen::EditRelease(screen) => match screen.update(message) {
                    add_release_main::Action::Back => self.create_main_screen(),
                    add_release_main::Action::ReleaseSubmitted => self.create_main_screen(),
                    add_release_main::Action::Run(task) => {
                        Action::Run(task.map(Message::EditReleaseScreen))
                    }
                    add_release_main::Action::None => Action::None,
                    add_release_main::Action::Error(error) => Action::Error(error),
                },
                _ => Action::None,
            },
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        match &self.screen {
            ViewGameScreen::ViewGame(screen) => screen.view().map(Message::ViewGameScreen),
            // TODO should this be in ViewReleaseMain?
            ViewGameScreen::ViewImage(screen) => screen.view().map(Message::ViewImageScreen),
            ViewGameScreen::ViewRelease(screen) => screen.view().map(Message::ViewReleaseScreen),
            ViewGameScreen::EditRelease(screen) => screen.view().map(Message::EditReleaseScreen),
        }
    }

    fn create_main_screen(&mut self) -> Action {
        let screen = view_game::ViewGame::new(self.game_id);
        match screen {
            Ok(screen) => {
                self.screen = ViewGameScreen::ViewGame(screen);
                Action::None
            }
            Err(e) => Action::Error(e),
        }
    }
}
