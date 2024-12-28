pub mod add_release_main;
pub mod add_release_screen;
pub mod error;
pub mod games_main;
pub mod games_screen;
pub mod home;
pub mod manage_emulators;
pub mod manage_games;
pub mod manage_systems;
pub mod settings_main;
pub mod settings_screen;
pub mod view_game;
pub mod view_game_main;
pub mod view_game_screen;
pub mod view_image;
pub mod view_release;
pub use add_release_main::AddReleaseMain;
pub use error::Error;
pub use games_main::GamesMain;
pub use home::Home;
pub use manage_emulators::ManageEmulators;
pub use manage_games::ManageGames;
pub use manage_systems::ManageSystems;
pub use settings_main::SettingsMain;

pub enum Screen {
    Home(Home),
    AddReleaseMain(AddReleaseMain),
    ManageSystems(ManageSystems),
    ManageGames(ManageGames),
    ManageEmulators(ManageEmulators),
    Error(Error),
    GamesMain(GamesMain),
    SettingsMain(SettingsMain),
}
