use bevy::prelude::{App, AppExtStates, Startup, Update};
// use bevy_asset_loader::prelude::*;

use bevy_asset_loader::loading_state::{
    config::ConfigureLoadingState, LoadingState, LoadingStateAppExt,
};
use states::{game::InGameAssets, AppState};

mod components;
mod constants;
mod game;
mod states;
mod types;
mod utils;
mod window;

fn main() {
    App::new()
        .add_plugins(game::bevy_default_set())
        .init_state::<AppState>()
        // .init_state::<RunningState>()
        .add_plugins(states::MainMenuPlugin)
        .add_plugins(states::GamePlugin)
        .add_plugins(states::GameOverPlugin)
        .add_loading_state(
            LoadingState::new(AppState::LoadGame)
                .continue_to_state(AppState::Game)
                .load_collection::<InGameAssets>(),
        )
        //.add_plugins(board::BoardPlugin)
        //.add_plugins(tetromino::TetrominoPlugin)
        .add_systems(Startup, game::setup)
        .add_systems(Update, game::update)
        // .add_systems(Update, window::close_on_q)
        .add_systems(Update, game::close_on_q)
        //.add_systems(Startup, state::main_menu::setup)
        .run();
}
