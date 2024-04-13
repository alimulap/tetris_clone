use bevy::{prelude::{*, App, Startup, Update}, window::close_on_esc};
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
        .init_state::<AppState>()
        .add_plugins(states::MainMenuPlugin)
        .add_plugins(states::GamePlugin)
        //.add_plugins(board::BoardPlugin)
        //.add_plugins(tetromino::TetrominoPlugin)
        .add_systems(Startup, game::setup)
        .add_systems(Update, game::update)
        .add_systems(Update, close_on_esc)
        .add_systems(Update, close_on_q)
        //.add_systems(Startup, state::main_menu::setup)
        .run();
}

pub fn close_on_q(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::KeyQ) {
            commands.entity(window).despawn();
        }
    }
}
