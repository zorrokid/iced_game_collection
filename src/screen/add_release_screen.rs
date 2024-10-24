pub mod add_release_main_screen;
pub use crate::manage_games::ManageGames;
pub use crate::manage_systems::ManageSystems;
pub use add_release_main_screen::AddReleaseMainScreen;

#[derive(Debug, Clone)]
pub enum AddReleaseScreen {
    AddReleaseMainScreen(AddReleaseMainScreen),
    ManageGamesScreen(ManageGames),
    ManageSystemsScreen(ManageSystems),
}
