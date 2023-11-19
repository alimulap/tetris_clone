use bevy::prelude::App;

mod game;
mod board;
mod tetromino;
mod state;
mod constants;
mod utils;

fn main() {
    App::new()
        .add_plugins(game::bevy_default_set())
        .add_plugins(board::BoardPlugin)
        .add_plugins(tetromino::TetrominoPlugin)
        .run();
}
