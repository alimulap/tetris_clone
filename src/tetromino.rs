#![allow(unused)]

use bevy::prelude::*;

use crate::{constants::{TETROMINO_SIZE, BOARD_POSITION}, board::{self, BlocksInBoard}, utils::LayoutParse};

pub struct TetrominoPlugin;

impl Plugin for TetrominoPlugin {
    fn build(&self, app: &mut App) {
    }
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

pub fn setup(_commands: &mut Commands, _asset_server: Res<AssetServer>) {
    let asd = Vec2::new(1.0, 1.0);
    let aqqwe = asd.y.atan2(asd.x);
}

pub fn spawn_tetromino(tetromino: Tetromino, blocks_in_board: &BlocksInBoard, commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let texture = asset_server.load("tetromino.png");

    let board_topleft = board::topleft();

    let pos = if let Tetromino::I = tetromino {
        (3, 19)
    } else {
        (4, 19)
    };

    let layout: Vec<Vec<u8>> = match tetromino {
        Tetromino::I => crate::constants::I_OVERLAY[0].parse(),
        Tetromino::O => crate::constants::O_OVERLAY[0].parse(),
        Tetromino::T => crate::constants::T_OVERLAY[0].parse(),
        Tetromino::S => crate::constants::S_OVERLAY[0].parse(),
        Tetromino::Z => crate::constants::Z_OVERLAY[0].parse(),
        Tetromino::J => crate::constants::J_OVERLAY[0].parse(),
        Tetromino::L => crate::constants::L_OVERLAY[0].parse(),
    };

    if !board::valid_in_board(blocks_in_board, &layout, pos) {
        println!("Game Over!"); //lmao
        return;
    }

    for (y, row) in layout.iter().enumerate() {
        for (x, block) in row.iter().enumerate() {
            if *block == 1 {
            }
        }
    }

    let x = board_topleft.x + (pos.0 as f32 + 0.5) * TETROMINO_SIZE.x;
    let y = board_topleft.y - (pos.1 as f32 + 0.5) * TETROMINO_SIZE.y;

    commands.spawn((
        tetromino,
        SpriteBundle {
            texture,
            transform: Transform { 
                translation: Vec3::new(x, y, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(TETROMINO_SIZE),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}
