pub mod games_main_screen;
pub use games_main_screen::GamesMainScreen;

use super::view_game_main::ViewGameMain;

#[derive(Debug, Clone)]
pub enum GamesScreen {
    GamesMainScreen(GamesMainScreen),
    ViewGameScreen(ViewGameMain),
}
