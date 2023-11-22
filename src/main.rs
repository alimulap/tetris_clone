use bevy::prelude::{App, Startup, Update};
use states::AppState;

mod game;
mod states;
mod components;
mod constants;
mod utils;
mod types;

fn main() {
    App::new()
        .add_plugins(game::bevy_default_set())
        .add_state::<AppState>()
        .add_plugins(states::MainMenuPlugin)
        .add_plugins(states::GamePlugin)
        //.add_plugins(board::BoardPlugin)
        //.add_plugins(tetromino::TetrominoPlugin)
        .add_systems(Startup, game::setup)
        .add_systems(Update, game::update)
        //.add_systems(Startup, state::main_menu::setup)
        .run();
}
