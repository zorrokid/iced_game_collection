pub mod add_game;
pub mod game_details;
pub mod games;
pub mod home;

pub use add_game::AddGame;
pub use game_details::GameDetails;
pub use games::Games;
pub use home::Home;

pub enum Screen {
    Home(Home),
    AddGame(AddGame),
    Games(Games),
    GameDetails(GameDetails),
}
