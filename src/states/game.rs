use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::components::{
    board::{self, BlocksInBoard},
    tetromino::{
        hard_drop_handler, move_tetromino, rotate_tetromino, MoveDirection, RotateDirection,
        ShouldHardDrop,
    },
};

use super::AppState;

//pub struct Game;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), (setup, board::setup.after(setup)))
            // .add_systems(OnEnter(AppState::Game), board::setup)
            .add_systems(
                Update,
                (
                    input_handler,
                    move_tetromino,
                    rotate_tetromino,
                    timer_ticker,
                    hard_drop_handler,
                    board::merge_blocks,
                )
                    .chain()
                    .run_if(in_state(AppState::Game))
                    // .run_if(in_state(RunningState::Running)),
            )
            .add_systems(OnExit(AppState::Game), cleanup);
    }
}

#[derive(AssetCollection, Resource)]
pub struct InGameAssets {
    #[asset(path = "board.png")]
    pub board_tex: Handle<Image>,
    #[asset(path = "tetromino.png")]
    pub tetromino_tex: Handle<Image>,
}

#[derive(Resource)]
struct MoveTimer(Timer);

#[derive(Resource)]
pub struct DropTimer(Timer);

impl DropTimer {
    pub fn start(&mut self) {
        self.0.unpause();
        self.0.reset();
    }

    pub fn restart(&mut self) {
        self.0.reset();
    }

    pub fn pause(&mut self) {
        self.0.pause();
    }

    pub fn paused(&self) -> bool {
        self.0.paused()
    }
}

#[derive(Resource)]
struct HoldTimer(Timer);

#[derive(Resource)]
struct PressedTimer(Timer);

#[derive(Resource)]
struct IsHolding(bool);

#[derive(Resource)]
struct KeyHolds {
    left: bool,
    right: bool,
    down: bool,
    //up: bool,
    //space: bool,
}

impl KeyHolds {
    fn new() -> Self {
        Self {
            left: false,
            right: false,
            down: false,
            //up: false,
            //space: false,
        }
    }
}

#[derive(Resource)]
pub struct ShouldMerge(pub bool);

fn setup(mut commands: Commands) {
    commands.insert_resource(BlocksInBoard::new());
    commands.insert_resource(MoveDirection::None);
    commands.insert_resource(HoldTimer(Timer::from_seconds(0.1, TimerMode::Repeating)));
    commands.insert_resource(PressedTimer(Timer::from_seconds(
        0.05,
        TimerMode::Repeating,
    )));
    commands.insert_resource(MoveTimer(Timer::from_seconds(1.2, TimerMode::Repeating)));
    commands.insert_resource(IsHolding(false));
    commands.insert_resource(KeyHolds::new());
    let mut drop_timer = Timer::from_seconds(0.5, TimerMode::Once);
    drop_timer.pause();
    commands.insert_resource(DropTimer(drop_timer));
    commands.insert_resource(ShouldMerge(false));
    commands.insert_resource(RotateDirection::None);
    commands.insert_resource(ShouldHardDrop(false));
    // commands.remove_resource();
}

fn cleanup(mut commands: Commands, board_query: Query<Entity, With<board::Board>>) {
    commands.remove_resource::<BlocksInBoard>();
    commands.remove_resource::<MoveDirection>();
    commands.remove_resource::<MoveTimer>();
    commands.remove_resource::<DropTimer>();
    commands.remove_resource::<HoldTimer>();
    commands.remove_resource::<PressedTimer>();
    commands.remove_resource::<IsHolding>();
    commands.remove_resource::<KeyHolds>();
    commands.remove_resource::<ShouldMerge>();
    commands.remove_resource::<RotateDirection>();
    commands.remove_resource::<ShouldHardDrop>();
    commands.remove_resource::<InGameAssets>();
    commands.entity(board_query.single()).despawn_recursive();
}

fn timer_ticker(
    time: Res<Time>,
    mut move_timer: ResMut<MoveTimer>,
    mut drop_timer: ResMut<DropTimer>,
    mut direction: ResMut<MoveDirection>,
    mut should_merge: ResMut<ShouldMerge>,
) {
    if move_timer.0.tick(time.delta()).just_finished() && *direction == MoveDirection::None {
        *direction = MoveDirection::Down;
    }

    if drop_timer.0.tick(time.delta()).just_finished() && **should_merge == false {
        *should_merge = ShouldMerge(true);
        drop_timer.restart();
        drop_timer.pause();
    }
}

fn input_handler(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut move_direction: ResMut<MoveDirection>,
    mut rotate_direction: ResMut<RotateDirection>,
    mut hold_timer: ResMut<HoldTimer>,
    mut pressed_timer: ResMut<PressedTimer>,
    mut drop_timer: ResMut<DropTimer>,
    mut is_holding: ResMut<KeyHolds>,
    mut should_hard_drop: ResMut<ShouldHardDrop>,
) {
    match (
        **should_hard_drop,
        keyboard_input.just_pressed(KeyCode::Space),
    ) {
        (false, true) => {
            **should_hard_drop = true;
            hold_timer.0.reset();
            pressed_timer.0.reset();
            if !drop_timer.paused() {
                drop_timer.restart();
                drop_timer.pause();
            }
            is_holding.right = false;
            is_holding.left = false;
            is_holding.down = false;
        }
        (false, false) => {
            if keyboard_input.pressed(KeyCode::ArrowRight) {
                if keyboard_input.just_pressed(KeyCode::ArrowRight) {
                    hold_timer.0.reset();
                    pressed_timer.0.reset();
                    *move_direction = MoveDirection::Right;
                } else if !is_holding.right && hold_timer.0.tick(time.delta()).just_finished() {
                    is_holding.right = true;
                } else if is_holding.right && pressed_timer.0.tick(time.delta()).just_finished() {
                    *move_direction = MoveDirection::Right;
                }
            } else if keyboard_input.pressed(KeyCode::ArrowLeft) {
                if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
                    hold_timer.0.reset();
                    pressed_timer.0.reset();
                    *move_direction = MoveDirection::Left;
                } else if !is_holding.left && hold_timer.0.tick(time.delta()).just_finished() {
                    is_holding.left = true;
                } else if is_holding.left && pressed_timer.0.tick(time.delta()).just_finished() {
                    *move_direction = MoveDirection::Left;
                }
            } else if keyboard_input.pressed(KeyCode::ArrowDown) {
                if keyboard_input.just_pressed(KeyCode::ArrowDown) {
                    hold_timer.0.reset();
                    pressed_timer.0.reset();
                    *move_direction = MoveDirection::Down;
                } else if !is_holding.down && hold_timer.0.tick(time.delta()).just_finished() {
                    is_holding.down = true;
                } else if is_holding.down && pressed_timer.0.tick(time.delta()).just_finished() {
                    *move_direction = MoveDirection::Down;
                }
            } else {
                hold_timer.0.reset();
                pressed_timer.0.reset();
            }

            if keyboard_input.just_released(KeyCode::ArrowRight) {
                is_holding.right = false;
            }
            if keyboard_input.just_released(KeyCode::ArrowLeft) {
                is_holding.left = false;
            }
            if keyboard_input.just_released(KeyCode::ArrowDown) {
                is_holding.down = false;
            }

            if keyboard_input.just_pressed(KeyCode::ArrowUp) {
                *rotate_direction = RotateDirection::Clockwise;
            } else if keyboard_input.just_pressed(KeyCode::KeyQ) {
                *rotate_direction = RotateDirection::CounterClockwise;
            }
        }
        _ => (),
    }
}
