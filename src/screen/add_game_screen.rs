pub mod add_game_main_screen;
pub mod manage_releases_screen;
pub use add_game_main_screen::AddGameMainScreen;
pub use manage_releases_screen::ManageReleasesScreen;

#[derive(Debug, Clone)]
pub enum AddGameScreen {
    AddGameMainScreen(AddGameMainScreen),
    ManageReleasesScreen(ManageReleasesScreen),
}
