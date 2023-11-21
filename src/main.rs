use bevy::prelude::{App, Startup};
use state::{AppState, MainMenuPlugin};

mod game;
mod board;
mod tetromino;
mod state;
mod constants;
mod utils;
mod types;

fn main() {
    App::new()
        .add_plugins(game::bevy_default_set())
        .add_state::<AppState>()
        .add_plugins(MainMenuPlugin)
        .add_plugins(board::BoardPlugin)
        .add_plugins(tetromino::TetrominoPlugin)
        .add_systems(Startup, game::setup)
        //.add_systems(Startup, state::main_menu::setup)
        .run();
}
