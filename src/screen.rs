pub mod add_emulator;
pub mod add_game_main;
pub mod add_game_screen;
pub mod add_system;
pub mod games;
pub mod home;
pub use add_emulator::AddEmulator;
pub use add_game_main::AddGameMain;
pub use add_system::AddSystem;
pub use games::Games;
pub use home::Home;

pub enum Screen {
    Home(Home),
    Games(Games),
    AddGameMain(AddGameMain),
    AddSystem(AddSystem),
    AddEmulator(AddEmulator),
}
