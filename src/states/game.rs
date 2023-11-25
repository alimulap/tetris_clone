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
            .add_systems(OnEnter(AppState::Game), setup)
            .add_systems(OnEnter(AppState::Game), board::setup)
            .add_systems(
                Update,
                (input_handler, move_tetromino, timer_ticker, board::merge_blocks).chain().run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), cleanup);
    }
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
    commands.insert_resource(MoveDirection::None);
    commands.insert_resource(HoldTimer(Timer::from_seconds(0.15, TimerMode::Repeating)));
    commands.insert_resource(PressedTimer(Timer::from_seconds(0.05, TimerMode::Repeating)));
    commands.insert_resource(MoveTimer(Timer::from_seconds(1.2, TimerMode::Repeating)));
    commands.insert_resource(IsHolding(false));
    commands.insert_resource(KeyHolds::new());
    let mut drop_timer = Timer::from_seconds(0.5, TimerMode::Once);
    drop_timer.pause();
    commands.insert_resource(DropTimer(drop_timer));
    commands.insert_resource(ShouldMerge(false));
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
        println!("drop timer finished");
        *should_merge = ShouldMerge(true);
        drop_timer.pause();
    }
}

fn input_handler(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut direction: ResMut<MoveDirection>,
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
}


