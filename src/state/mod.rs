use bevy::prelude::States;

pub mod main_menu;
pub use main_menu::MainMenuPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
