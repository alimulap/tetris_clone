#![allow(unused)]

use bevy::{prelude::*, sprite::Anchor};

use crate::{
    board::{self, BlocksInBoard, Board},
    constants::{BOARD_POSITION, TETROMINO_SIZE, BOARD_BORDER_THICKNESS},
    utils::LayoutParse, state::AppState,
};

pub struct TetrominoPlugin;

impl Plugin for TetrominoPlugin {
    fn build(&self, _app: &mut App) {}
}

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
pub struct TetrominoPosition(pub [i8; 2]);

pub fn _setup(_commands: &mut Commands, _asset_server: Res<AssetServer>) {}

pub fn spawn_tetromino(
    tetromino: Tetromino,
    blocks_in_board: &BlocksInBoard,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    board: Entity,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let asd = TetrominoPosition([0, 0]);
    let texture = asset_server.load("tetromino.png");

    let pos = match tetromino {
        Tetromino::O => (4,  0),
        Tetromino::I => (3, -1),
        _ =>            (3,  0),
    };
    let layout: Vec<Vec<u8>> = match tetromino {
        Tetromino::I => crate::constants::I_LAYOUT[0].parse(),
        Tetromino::O => crate::constants::O_LAYOUT[0].parse(),
        Tetromino::T => crate::constants::T_LAYOUT[0].parse(),
        Tetromino::S => crate::constants::S_LAYOUT[0].parse(),
        Tetromino::Z => crate::constants::Z_LAYOUT[0].parse(),
        Tetromino::J => crate::constants::J_LAYOUT[0].parse(),
        Tetromino::L => crate::constants::L_LAYOUT[0].parse(),
    };

    if !board::valid_in_board(blocks_in_board, &layout, &pos) {
        next_state.set(AppState::GameOver);
        return;
    }

    let tetromino_position = Vec3::new(
        BOARD_BORDER_THICKNESS + TETROMINO_SIZE.x * pos.0 as f32,
        -BOARD_BORDER_THICKNESS - TETROMINO_SIZE.y * pos.1 as f32,
        0.,
    );
    println!("tetromino_position: {:?}", tetromino_position);

    let tetromino = commands
        .spawn((
            tetromino,
            TransformBundle {
                local: Transform::from_translation(tetromino_position),
                global: GlobalTransform::default()
            },
            VisibilityBundle::default(),
        ))
        .with_children(|parent| {
            for (y, row) in layout.iter().enumerate() {
                for (x, block) in row.iter().enumerate() {
                    if *block == 1 {
                        spawn_block(
                            parent,
                            texture.clone(),
                            x as f32,
                            y as f32,
                        );
                    }
                }
            }
        }).id();

    commands.entity(board).add_child(tetromino);
}

pub fn spawn_block(
    parent: &mut ChildBuilder,
    texture: Handle<Image>,
    x: f32,
    y: f32,
) {
    let pos_x = x * TETROMINO_SIZE.x;
    let pos_y = y * TETROMINO_SIZE.y;
    println!("pos_x: {}, pos_y: {}", pos_x, pos_y);

    parent.spawn((
        Block,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(TETROMINO_SIZE),
                anchor: Anchor::TopLeft,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(pos_x, -pos_y, 0.0)),
            texture,
            ..Default::default()
        },
    ));
}
