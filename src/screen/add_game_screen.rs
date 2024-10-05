pub mod add_game_main_screen;
pub mod add_release_screen;
pub use add_game_main_screen::AddGameMainScreen;
pub use add_release_screen::AddReleaseScreen;

#[derive(Debug, Clone)]
pub enum AddGameScreen {
    AddGameMainScreen(AddGameMainScreen),
    AddReleaseScreen(AddReleaseScreen),
}
