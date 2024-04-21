use bevy::prelude::States;

pub mod main_menu;
pub use main_menu::MainMenuPlugin;

pub mod game;
pub use game::GamePlugin;

pub mod game_over;
pub use game_over::GameOverPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    LoadGame,
    Game,
    GameOver,
}

//#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
//pub enum PlayerState {
//    #[default]
//    Idle,
//    #[allow(dead_code)]
//    Walk,
//    #[allow(dead_code)]
//    Run
//}

// #[allow(dead_code)]
// #[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
// pub enum RunningState {
//     Loading,
//     #[default]
//     Running,
//     Pause,
// }
