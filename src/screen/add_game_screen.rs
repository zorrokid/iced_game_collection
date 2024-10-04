pub mod sub_screen;
pub mod sub_screen2;
pub use sub_screen::SubScreen;
pub use sub_screen2::SubScreen2;

#[derive(Debug, Clone)]
pub enum AddGameScreen {
    SubScreen(SubScreen),
    SubScreen2(sub_screen2::SubScreen2),
}
