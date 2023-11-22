use std::ops::{Deref, DerefMut};

use bevy::{prelude::*, sprite::Anchor};

use crate::{
    constants::{BOARD_OUTER_SIZE, BOARD_POSITION},
    states::AppState,
    types::Position,
};

use super::tetromino::{spawn_tetromino, Tetromino};

#[derive(Component)]
pub struct Board;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    blocks_in_board: Res<BlocksInBoard>,
    next_state: ResMut<NextState<AppState>>,
) {
    let texture = asset_server.load("board.png");
    let board = commands
        .spawn((
            Board,
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: BOARD_POSITION.extend(0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    custom_size: Some(BOARD_OUTER_SIZE),
                    anchor: Anchor::TopLeft,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    spawn_tetromino(
        Tetromino::J,
        &blocks_in_board,
        &mut commands,
        &asset_server,
        board,
        next_state,
    );
}

#[derive(Resource)]
pub struct BlocksInBoard(Vec<Vec<u8>>);

impl BlocksInBoard {
    pub fn new() -> Self {
        Self(vec![vec![0; 10]; 20])
    }
}

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

pub fn valid_in_board(
    blocks_in_board: &BlocksInBoard,
    layout: &Vec<Vec<u8>>,
    pos: &Position,
) -> bool {
    for (y, row) in layout.iter().enumerate() {
        for (x, block) in row.iter().enumerate() {
            if *block == 1 {
                let x = x as i32 + pos.x;
                let y = y as i32 + pos.y;

                if x < 0
                    || x >= 10
                    || y < 0
                    || y >= 20
                    || blocks_in_board[y as usize][x as usize] == 1
                {
                    return false;
                }
            }
        }
    }

    return true;
}