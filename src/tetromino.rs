#![allow(unused)]

use bevy::prelude::*;

use crate::{
    board::{self, BlocksInBoard},
    constants::{BOARD_POSITION, TETROMINO_SIZE},
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

pub fn setup(_commands: &mut Commands, _asset_server: Res<AssetServer>) {}

pub fn spawn_tetromino(
    tetromino: Tetromino,
    blocks_in_board: &BlocksInBoard,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let texture = asset_server.load("tetromino.png");

    let board_topleft = board::topleft();
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
        println!("Game Over!"); //lmao
        return;
    }

    commands
        .spawn((
            tetromino,
            TransformBundle::default(),
            VisibilityBundle::default(),
        ))
        .with_children(|parent| {
            println!("Tetromino");
            for (y, row) in layout.iter().enumerate() {
                for (x, block) in row.iter().enumerate() {
                    if *block == 1 {
                        spawn_block(
                            parent,
                            texture.clone(),
                            &board_topleft,
                            &pos,
                            x as f32,
                            y as f32,
                        );
                    }
                }
            }
        });
}
pub fn spawn_block(
    parent: &mut ChildBuilder,
    texture: Handle<Image>,
    board_topleft: &Vec2,
    pos: &(i32, i32),
    x: f32,
    y: f32,
) {
    println!("Block");
    let tetromino_size = TETROMINO_SIZE / 4.;
    let pos_x = board_topleft.x + (pos.0 as f32 + x + 0.5) * tetromino_size.x;
    let pos_y = board_topleft.y - (pos.1 as f32 + y + 0.5) * tetromino_size.y;
    println!("pos_x: {}, pos_y: {}", pos_x, pos_y);

    parent.spawn((
        Block,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(tetromino_size),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(pos_x, pos_y, 0.0)),
            texture,
            ..Default::default()
        },
    ));
}
