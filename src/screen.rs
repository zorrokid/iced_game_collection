pub mod add_game;
pub mod add_game_main;
pub mod add_game_screen;
pub mod add_release;
pub mod add_system;
pub mod game_details;
pub mod games;
pub mod home;
pub use add_game::AddGame;
pub use add_game_main::AddGameMain;
pub use add_release::AddRelease;
pub use add_system::AddSystem;
pub use game_details::GameDetails;
pub use games::Games;
pub use home::Home;

pub enum Screen {
    Home(Home),
    AddGame(AddGame),
    Games(Games),
    GameDetails(GameDetails),
    AddGameMain(AddGameMain),
    AddSystem(AddSystem),
    AddRelease(AddRelease),
}
