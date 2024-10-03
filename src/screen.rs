pub mod add_game;
pub mod add_system;
pub mod game_details;
pub mod games;
pub mod home;
pub use add_game::AddGame;
pub use add_system::AddSystem;
pub use game_details::GameDetails;
pub use games::Games;
pub use home::Home;

pub enum Screen {
    Home(Home),
    AddGame(AddGame),
    Games(Games),
    AddSystem(AddSystem),
    GameDetails(GameDetails),
}
