use bevy::{app::Plugin, prelude::States};

pub mod mainmenu;
pub use mainmenu::MainMenuPlugin;

pub mod ingame;
pub use ingame::GamePlugin;

pub mod gameover;
pub use gameover::GameOverPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    LoadGame,
    Game,
    GameOver,
}

pub struct Screens;

impl Plugin for Screens {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((MainMenuPlugin, GamePlugin, GameOverPlugin));
    }
}
