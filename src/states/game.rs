use std::time::Duration;

use bevy::prelude::*;

use crate::components::{
    board::{self, BlocksInBoard},
    tetromino::{move_tetromino, MoveDirection},
};

use super::AppState;

//pub struct Game;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BlocksInBoard::new())
            .insert_resource(MoveDirection::None)
            .add_systems(OnEnter(AppState::Game), setup)
            .add_systems(OnEnter(AppState::Game), board::setup)
            .add_systems(
                Update,
                (move_timer, move_tetromino, input_handler).run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), cleanup);
    }
}

#[derive(Resource)]
struct MoveTimer(Timer);

#[derive(Resource)]
struct DropTimer(Timer);

fn setup(mut commands: Commands) {
    let mut drop_timer = Timer::from_seconds(5.5, TimerMode::Once);
    drop_timer.set_elapsed(Duration::from_secs_f32(12.5));
    //drop_timer.
    commands.insert_resource(MoveTimer(Timer::from_seconds(1.2, TimerMode::Repeating)));
    commands.insert_resource(DropTimer(drop_timer));
}

fn cleanup(mut commands: Commands, board_query: Query<Entity, With<board::Board>>) {
    commands.remove_resource::<MoveTimer>();
    commands.remove_resource::<DropTimer>();
    commands.entity(board_query.single()).despawn_recursive();
}

fn move_timer(
    time: Res<Time>,
    mut move_timer: ResMut<MoveTimer>,
    mut drop_timer: ResMut<DropTimer>,
    mut direction: ResMut<MoveDirection>,
) {
    if move_timer.0.tick(time.delta()).just_finished() && *direction == MoveDirection::None {
        *direction = MoveDirection::Down;
    }

    if drop_timer.0.tick(time.delta()).just_finished() {
        println!("Drop");
    }
}

fn input_handler(keyboard_input: Res<Input<KeyCode>>, mut direction: ResMut<MoveDirection>) {
    if *direction != MoveDirection::None {
        return;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        *direction = MoveDirection::Left;
    } else if keyboard_input.pressed(KeyCode::Right) {
        *direction = MoveDirection::Right;
    } else if keyboard_input.pressed(KeyCode::Down) {
        *direction = MoveDirection::Down;
    }
}
