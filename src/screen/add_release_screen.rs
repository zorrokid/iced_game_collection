pub mod add_release_main_screen;
pub mod manage_games;
pub use crate::manage_systems::ManageSystems;
pub use add_release_main_screen::AddReleaseMainScreen;
pub use manage_games::ManageGames;

#[derive(Debug, Clone)]
pub enum AddReleaseScreen {
    AddReleaseMainScreen(AddReleaseMainScreen),
    ManageGamesScreen(ManageGames),
    ManageSystemsScreen(ManageSystems),
}
