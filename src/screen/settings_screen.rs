use settings_main_screen::SettingsMainScreen;

pub mod settings_main_screen;
pub mod settings_widget;

#[derive(Debug, Clone)]
pub enum SettingsScreen {
    SettingsMainScreen(SettingsMainScreen),
}
