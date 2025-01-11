use bevy::prelude::{App, AppExtStates, Startup, Update};
// use bevy_asset_loader::prelude::*;

use screens::AppState;

mod constants;
mod game;
mod modules;
mod screens;
mod types;
mod utils;

fn main() {
    App::new()
        .add_plugins(game::bevy_default_set())
        .init_state::<AppState>()
        // .init_state::<RunningState>()
        .add_plugins(screens::Screens)
        .add_plugins(modules::button::ButtonPlugin)
        .add_plugins(modules::cursor::CursorPlugin)
        .add_systems(Startup, game::setup)
        .add_systems(Update, game::close_on_q)
        .run();
}
