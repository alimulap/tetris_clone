use bevy::{prelude::*, sprite::Anchor};

use crate::{
    constants::{BOARD_BORDER_THICKNESS, TETROMINO_SIZE},
    states::{
        game::{GameTimers, ShouldMerge},
        AppState,
    },
    types::Position,
    utils::LayoutParse,
};

use super::board::{valid_in_board, BlocksInBoard};

#[allow(unused)]
#[derive(Component, Clone, Copy)]
pub enum Tetromino {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

#[derive(Component)]
pub struct Block;

#[derive(Component)]
pub struct IndexLayout(pub usize);

pub fn spawn_tetromino(
    tetromino: Tetromino,
    blocks_in_board: &BlocksInBoard,
    commands: &mut Commands,
    texture: &Handle<Image>,
    board: Entity,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let pos = if let Tetromino::O = tetromino {
        Position::new(4, 0)
    } else {
        Position::new(3, 0)
    };
    let index = IndexLayout(0);
    let layout: Vec<Vec<u8>> = match tetromino {
        Tetromino::I => crate::constants::I_LAYOUT[*index].parse(),
        Tetromino::O => crate::constants::O_LAYOUT[*index].parse(),
        Tetromino::T => crate::constants::T_LAYOUT[*index].parse(),
        Tetromino::S => crate::constants::S_LAYOUT[*index].parse(),
        Tetromino::Z => crate::constants::Z_LAYOUT[*index].parse(),
        Tetromino::J => crate::constants::J_LAYOUT[*index].parse(),
        Tetromino::L => crate::constants::L_LAYOUT[*index].parse(),
    };

    if valid_in_board(blocks_in_board, &layout, &pos) {
        let tetromino_position = Vec3::new(
            BOARD_BORDER_THICKNESS + TETROMINO_SIZE.x * pos.x as f32,
            -BOARD_BORDER_THICKNESS - TETROMINO_SIZE.y * pos.y as f32,
            0.,
        );

        let tetromino = commands
            .spawn((
                tetromino,
                pos,
                index,
                Transform::from_translation(tetromino_position),
                InheritedVisibility::default(),
            ))
            .with_children(|parent| {
                for (y, row) in layout.iter().enumerate() {
                    for (x, block) in row.iter().enumerate() {
                        if *block == 1 {
                            spawn_block(parent, texture.clone(), x, y);
                        }
                    }
                }
            })
            .id();

        commands.entity(board).add_child(tetromino);
    } else {
        next_state.set(AppState::GameOver);
    }
}

pub fn spawn_block(parent: &mut ChildBuilder, texture: Handle<Image>, x: usize, y: usize) {
    let pos = Position::new(x as i32, y as i32);
    parent.spawn((
        Block,
        pos,
        Sprite {
            custom_size: Some(TETROMINO_SIZE),
            anchor: Anchor::TopLeft,
            image: texture,
            ..Default::default()
        },
        Transform::from_translation(Vec3 {
            x: x as f32 * TETROMINO_SIZE.x,
            y: y as f32 * -TETROMINO_SIZE.y,
            z: 0.,
        }),
    ));
}

#[allow(unused)]
#[derive(Resource, PartialEq)]
pub enum MoveDirection {
    Left,
    Right,
    Down,
    None,
}

#[derive(Resource, PartialEq)]
pub enum RotateDirection {
    Clockwise,
    CounterClockwise,
    None,
}

#[derive(Resource)]
pub struct ShouldHardDrop(pub bool);

pub fn move_tetromino(
    mut query: Query<(&Tetromino, &mut Position, &mut Transform, &IndexLayout)>,
    blocks_in_board: Res<BlocksInBoard>,
    mut direction: ResMut<MoveDirection>,
    mut timers: ResMut<GameTimers>,
) {
    let pos_add = match *direction {
        MoveDirection::Left => Position::new(-1, 0),
        MoveDirection::Right => Position::new(1, 0),
        MoveDirection::Down => Position::new(0, 1),
        MoveDirection::None => return,
    };
    let (tetromino, mut pos, mut transform, index) = query.single_mut();
    let layout = match tetromino {
        Tetromino::I => crate::constants::I_LAYOUT[**index].parse(),
        Tetromino::O => crate::constants::O_LAYOUT[**index].parse(),
        Tetromino::T => crate::constants::T_LAYOUT[**index].parse(),
        Tetromino::S => crate::constants::S_LAYOUT[**index].parse(),
        Tetromino::Z => crate::constants::Z_LAYOUT[**index].parse(),
        Tetromino::J => crate::constants::J_LAYOUT[**index].parse(),
        Tetromino::L => crate::constants::L_LAYOUT[**index].parse(),
    };

    if valid_in_board(&blocks_in_board, &layout, &(*pos + pos_add)) {
        *pos += pos_add;
        transform.translation.x += TETROMINO_SIZE.x * pos_add.x as f32;
        transform.translation.y += TETROMINO_SIZE.y * -pos_add.y as f32;
        if !timers.drop_timer.paused()
            && (*direction == MoveDirection::Left || *direction == MoveDirection::Right)
        {
            timers.drop_timer.restart();
        }
    } else if *direction == MoveDirection::Down {
        timers.drop_timer.start();
    }

    *direction = MoveDirection::None;
}

pub fn rotate_tetromino(
    mut query: Query<(&Tetromino, &Position, &mut IndexLayout, &Children)>,
    mut tf_query: Query<(&mut Transform, &mut Position), Without<Tetromino>>,
    blocks_in_board: Res<BlocksInBoard>,
    mut direction: ResMut<RotateDirection>,
    mut timers: ResMut<GameTimers>,
) {
    let add_idx = match *direction {
        RotateDirection::Clockwise => 1,
        RotateDirection::CounterClockwise => -1,
        RotateDirection::None => return,
    };

    let (tetromino, pos, mut index, children) = query.single_mut();
    index.rotate(add_idx);

    let layout = match tetromino {
        Tetromino::I => crate::constants::I_LAYOUT[**index].parse(),
        Tetromino::O => crate::constants::O_LAYOUT[**index].parse(),
        Tetromino::T => crate::constants::T_LAYOUT[**index].parse(),
        Tetromino::S => crate::constants::S_LAYOUT[**index].parse(),
        Tetromino::Z => crate::constants::Z_LAYOUT[**index].parse(),
        Tetromino::J => crate::constants::J_LAYOUT[**index].parse(),
        Tetromino::L => crate::constants::L_LAYOUT[**index].parse(),
    };

    if valid_in_board(&blocks_in_board, &layout, pos) {
        let mut children = children.iter();
        for (y, row) in layout.iter().enumerate() {
            for (x, block) in row.iter().enumerate() {
                if *block == 1 {
                    let (mut tf, mut pos) = tf_query.get_mut(*children.next().unwrap()).unwrap();
                    tf.translation.x = x as f32 * TETROMINO_SIZE.x;
                    tf.translation.y = y as f32 * -TETROMINO_SIZE.y;
                    pos.x = x as i32;
                    pos.y = y as i32;
                }
            }
        }
        if !timers.drop_timer.paused()
            && (*direction == RotateDirection::Clockwise
                || *direction == RotateDirection::CounterClockwise)
        {
            timers.drop_timer.restart();
        }
        *direction = RotateDirection::None;
    }
}

pub fn hard_drop_handler(
    mut query: Query<(&Tetromino, &mut Position, &mut Transform, &IndexLayout)>,
    blocks_in_board: Res<BlocksInBoard>,
    mut should_hard_drop: ResMut<ShouldHardDrop>,
    mut should_merge: ResMut<ShouldMerge>,
) {
    if **should_hard_drop {
        let (tetromino, mut pos, mut transform, index) = query.single_mut();
        let layout = match tetromino {
            Tetromino::I => crate::constants::I_LAYOUT[**index].parse(),
            Tetromino::O => crate::constants::O_LAYOUT[**index].parse(),
            Tetromino::T => crate::constants::T_LAYOUT[**index].parse(),
            Tetromino::S => crate::constants::S_LAYOUT[**index].parse(),
            Tetromino::Z => crate::constants::Z_LAYOUT[**index].parse(),
            Tetromino::J => crate::constants::J_LAYOUT[**index].parse(),
            Tetromino::L => crate::constants::L_LAYOUT[**index].parse(),
        };

        let mut pos_add = Position::new(0, 0);

        while valid_in_board(
            &blocks_in_board,
            &layout,
            &pos.add_some_cloned(&[&pos_add, &Position::new(0, 1)]),
        ) {
            pos_add.y += 1;
        }

        *pos += pos_add;
        transform.translation.x += TETROMINO_SIZE.x * pos_add.x as f32;
        transform.translation.y += TETROMINO_SIZE.y * -pos_add.y as f32;

        **should_hard_drop = false;
        **should_merge = true;
    }
}
