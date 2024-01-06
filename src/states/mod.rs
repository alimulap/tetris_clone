use bevy::prelude::States;

pub mod main_menu;
pub use main_menu::MainMenuPlugin;

pub mod game;
pub use game::GamePlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum PlayerState {
    #[default]
    Idle,
    Walk,
    Run
}
