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

#[derive(Resource)]
struct HoldTimer(Timer);

#[derive(Resource)]
struct PressedTimer(Timer);

#[derive(Resource)]
struct IsHolding(bool);

#[allow(unused)]
#[derive(Resource)]
struct KeyHolds {
    left: bool,
    right: bool,
    down: bool,
    up: bool,
    space: bool,
}

fn setup(mut commands: Commands) {
    commands.insert_resource(HoldTimer(Timer::from_seconds(0.15, TimerMode::Repeating)));
    commands.insert_resource(PressedTimer(Timer::from_seconds(0.05, TimerMode::Repeating)));
    commands.insert_resource(MoveTimer(Timer::from_seconds(1.2, TimerMode::Repeating)));
    let mut drop_timer = Timer::from_seconds(0.5, TimerMode::Once);
    drop_timer.pause();
    commands.insert_resource(DropTimer(drop_timer));
    commands.insert_resource(IsHolding(false));
    commands.insert_resource(KeyHolds {
        left: false,
        right: false,
        down: false,
        up: false,
        space: false,
    });
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

fn input_handler(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut direction: ResMut<MoveDirection>,
    mut drop_timer: ResMut<DropTimer>,
    mut hold_timer: ResMut<HoldTimer>,
    mut pressed_timer: ResMut<PressedTimer>,
    mut is_holding: ResMut<KeyHolds>,
) {
    if keyboard_input.pressed(KeyCode::Right) {
        if keyboard_input.just_pressed(KeyCode::Right) {
            hold_timer.0.reset();
            pressed_timer.0.reset();
            *direction = MoveDirection::Right;
        } else if !is_holding.right && hold_timer.0.tick(time.delta()).just_finished() {
            is_holding.right = true;
        } else if is_holding.right && pressed_timer.0.tick(time.delta()).just_finished() {
            *direction = MoveDirection::Right;
        } 
    } else if keyboard_input.pressed(KeyCode::Left) {
        if keyboard_input.just_pressed(KeyCode::Left) {
            hold_timer.0.reset();
            pressed_timer.0.reset();
            *direction = MoveDirection::Left;
        } else if !is_holding.left && hold_timer.0.tick(time.delta()).just_finished() {
            is_holding.left = true;
        } else if is_holding.left && pressed_timer.0.tick(time.delta()).just_finished() {
            *direction = MoveDirection::Left;
        } 
    } else if keyboard_input.pressed(KeyCode::Down) {
        if keyboard_input.just_pressed(KeyCode::Down) {
            hold_timer.0.reset();
            pressed_timer.0.reset();
            *direction = MoveDirection::Down;
        } else if !is_holding.down && hold_timer.0.tick(time.delta()).just_finished() {
            is_holding.down = true;
        } else if is_holding.down && pressed_timer.0.tick(time.delta()).just_finished() {
            *direction = MoveDirection::Down;
        } 
    } else {
        hold_timer.0.reset();
        pressed_timer.0.reset();
    }

    if keyboard_input.just_released(KeyCode::Right) {
        is_holding.right = false;
    }
    if keyboard_input.just_released(KeyCode::Left) {
        is_holding.left = false;
    }
    if keyboard_input.just_released(KeyCode::Down) {
        is_holding.down = false;
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        drop_timer.0.reset();
        drop_timer.0.unpause();
    }
}
