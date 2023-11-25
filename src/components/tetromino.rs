use bevy::{prelude::*, sprite::Anchor};

use crate::{
    constants::{BOARD_BORDER_THICKNESS, TETROMINO_SIZE},
    states::{AppState, game::DropTimer},
    types::Position,
    utils::LayoutParse,
};

use super::board::{valid_in_board, BlocksInBoard};

#[allow(unused)]
#[derive(Component)]
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
    asset_server: &Res<AssetServer>,
    board: Entity,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let texture = asset_server.load("tetromino.png");

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

    if !valid_in_board(blocks_in_board, &layout, &pos) {
        next_state.set(AppState::GameOver);
        println!("Game Over");
        println!("layout: {:?}, pos: {:?}", layout, pos);
        return;
    }

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
            TransformBundle {
                local: Transform::from_translation(tetromino_position),
                global: GlobalTransform::default(),
            },
            VisibilityBundle::default(),
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

    println!("tetromino: {:?}", tetromino);

    commands.entity(board).add_child(tetromino);
}

pub fn spawn_block(parent: &mut ChildBuilder, texture: Handle<Image>, x: usize, y: usize) {
    let pos = Position::new(x as i32, y as i32);
    println!("spawn block pos: {:?}", pos);
    parent.spawn((
        Block,
        pos,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(TETROMINO_SIZE),
                anchor: Anchor::TopLeft,
                // color: Color::rgba(1., 1., 1., 0.5),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3 {
                x: x as f32 * TETROMINO_SIZE.x,
                y: y as f32 * -TETROMINO_SIZE.y,
                z: 0.,
            }),
            texture,
            ..Default::default()
        },
    ));
}

#[allow(unused)]
#[derive(Resource, PartialEq)]
pub enum MoveDirection {
    Left,
    Right,
    Down,
    None
}

pub fn move_tetromino(
    // mut commands: Commands,
    mut query: Query<(&Tetromino, &mut Position, &mut Transform, &IndexLayout)>,
    blocks_in_board: Res<BlocksInBoard>,
    mut direction: ResMut<MoveDirection>,
    mut drop_timer: ResMut<DropTimer>,
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
        if !drop_timer.paused() && (*direction == MoveDirection::Left || *direction == MoveDirection::Right) {
            drop_timer.restart();
        }
    } else if *direction == MoveDirection::Down {
        println!("Can't move down");
        drop_timer.start();
    }
    // else {
    //     println!("Can't move");
    // }

    *direction = MoveDirection::None;
}

