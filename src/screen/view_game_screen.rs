pub use super::{
    add_release_main::AddReleaseMain, view_game::ViewGame, view_image::ViewImage,
    view_release::ViewRelease,
};

#[derive(Debug, Clone)]
pub enum ViewGameScreen {
    ViewGame(ViewGame),
    ViewImage(ViewImage),
    ViewRelease(ViewRelease),
    EditRelease(AddReleaseMain),
}
