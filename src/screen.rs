pub mod add_game_main;
pub mod add_game_screen;
pub mod add_release_main;
pub mod add_release_screen;
pub mod error;
pub mod games;
pub mod home;
pub mod manage_emulators;
pub mod manage_systems;
pub mod view_game;
pub use add_game_main::AddGameMain;
pub use add_release_main::AddReleaseMain;
pub use error::Error;
pub use games::Games;
pub use home::Home;
pub use manage_emulators::ManageEmulators;
pub use manage_systems::ManageSystems;
pub use view_game::ViewGame;

pub enum Screen {
    Home(Home),
    Games(Games),
    AddGameMain(AddGameMain),
    AddReleaseMain(AddReleaseMain),
    ManageSystems(ManageSystems),
    ManageEmulators(ManageEmulators),
    ViewGame(ViewGame),
    Error(Error),
}
