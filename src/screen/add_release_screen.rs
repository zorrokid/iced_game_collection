pub mod add_release_main_screen;
pub mod manage_games_screen;
pub mod manage_systems_screen;
pub use add_release_main_screen::AddReleaseMainScreen;
pub use manage_games_screen::ManageGamesScreen;
pub use manage_systems_screen::ManageSystemsScreen;

#[derive(Debug, Clone)]
pub enum AddReleaseScreen {
    AddReleaseMainScreen(AddReleaseMainScreen),
    ManageGamesScreen(ManageGamesScreen),
    ManageSystemsScreen(ManageSystemsScreen),
}
