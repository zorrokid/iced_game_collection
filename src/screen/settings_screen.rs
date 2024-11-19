pub mod settings_main_screen;
pub use settings_main_screen::SettingsMainScreen;

#[derive(Debug, Clone)]
pub enum SettingsScreen {
    SettingsMainScreen(SettingsMainScreen),
}
