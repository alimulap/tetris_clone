use bevy::prelude::{App, Startup, Update};
use states::AppState;

mod components;
mod constants;
mod game;
mod states;
mod types;
mod utils;

fn main() {
    App::new()
        .add_plugins(game::bevy_default_set())
        .init_state::<AppState>()
        .add_plugins(states::MainMenuPlugin)
        .add_plugins(states::GamePlugin)
        .add_plugins(states::GameOverPlugin)
        //.add_plugins(board::BoardPlugin)
        //.add_plugins(tetromino::TetrominoPlugin)
        .add_systems(Startup, game::setup)
        .add_systems(Update, game::update)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, game::close_on_q)
        //.add_systems(Startup, state::main_menu::setup)
        .run();
}
