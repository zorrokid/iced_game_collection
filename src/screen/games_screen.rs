pub mod games_main_screen;
pub use crate::view_game::ViewGame;
pub use games_main_screen::GamesMainScreen;

#[derive(Debug, Clone)]
pub enum GamesScreen {
    GamesMainScreen(GamesMainScreen),
    ViewGameScreen(ViewGame),
}