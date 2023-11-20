#![allow(unused)]

use std::ops::{Deref, DerefMut};

use bevy::prelude::*;

use crate::{constants::{BOARD_POSITION, BOARD_INNER_SIZE, BOARD_OUTER_SIZE}, state::AppState, tetromino::{self, Tetromino}};

#[derive(Component)]
pub struct Board;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_systems(OnEnter(AppState::Game), setup)
            .insert_resource(BlocksInBoard(vec![vec![0; 20]; 10]));
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    blocks_in_board: Res<BlocksInBoard>,
) {
    commands.spawn(Camera2dBundle::default());

    let texture = asset_server.load("board.png");

    let board_position = BOARD_POSITION / 4.;
    let board_outer_size = BOARD_OUTER_SIZE / 4.;

    commands.spawn((
        Board,
        SpriteBundle {
            texture,
            transform: Transform {
                translation: board_position.extend(0.0),
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(board_outer_size),
                ..Default::default()
            },
            ..Default::default()
        },
    ));

    tetromino::spawn_tetromino(Tetromino::L, &blocks_in_board, &mut commands, &asset_server);
}

#[allow(unused)]
pub fn topleft() -> Vec2 {
    let board_position = BOARD_POSITION / 4.;
    let board_inner_size = BOARD_INNER_SIZE / 4.;
    Vec2::new(board_position.x - board_inner_size.x / 2., board_position.y + board_inner_size.y / 2.)
}

#[derive(Resource)]
pub struct BlocksInBoard(Vec<Vec<u8>>);

impl Deref for BlocksInBoard {
    type Target = Vec<Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BlocksInBoard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn valid_in_board(blocks_in_board: &BlocksInBoard, layout: &Vec<Vec<u8>>, pos: &(i32, i32)) -> bool {
    for (y, row) in layout.iter().enumerate() {
        for (x, block) in row.iter().enumerate() {
            if *block == 1 {
                let x = x as i32 + pos.0;
                let y = y as i32 + pos.1;
                println!("x: {}, y: {}", x, y);

                if x < 0 || x >= 10 || y < 0 || y >= 20 || blocks_in_board[y as usize][x as usize] == 1 {
                    return false;
                }
            }
        }
    }

    return true;
}


