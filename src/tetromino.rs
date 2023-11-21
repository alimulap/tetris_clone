#![allow(unused)]

use bevy::{prelude::*, sprite::Anchor};

use crate::{
    board::{self, BlocksInBoard, Board},
    constants::{BOARD_BORDER_THICKNESS, BOARD_POSITION, TETROMINO_SIZE},
    state::AppState,
    types::Position,
    utils::LayoutParse,
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
pub struct IndexLayout(pub usize);

pub fn _setup(_commands: &mut Commands, _asset_server: Res<AssetServer>) {}

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

    if !board::valid_in_board(blocks_in_board, &layout, &pos) {
        next_state.set(AppState::GameOver);
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
                        spawn_block(parent, texture.clone(), x as f32, y as f32);
                    }
                }
            }
        })
        .id();

    commands.entity(board).add_child(tetromino);
}

pub fn spawn_block(parent: &mut ChildBuilder, texture: Handle<Image>, x: f32, y: f32) {
    parent.spawn((
        Block,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(TETROMINO_SIZE),
                anchor: Anchor::TopLeft,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3 {
                x: x * TETROMINO_SIZE.x,
                y: y * -TETROMINO_SIZE.y,
                z: 0.,
            }),
            texture,
            ..Default::default()
        },
    ));
}
