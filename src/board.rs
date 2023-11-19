use std::ops::{Deref, DerefMut};

use bevy::prelude::*;

use crate::{constants::{BOARD_POSITION, BOARD_INNER_SIZE}, state::AppState};

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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("board.png");

    commands.spawn((
        Board,
        SpriteBundle {
            texture,
            transform: Transform {
                translation: BOARD_POSITION.extend(0.0),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

#[allow(unused)]
pub fn topleft() -> Vec2 {
    BOARD_POSITION - BOARD_INNER_SIZE / 2.
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

pub fn valid_in_board(blocks_in_board: &BlocksInBoard, layout: &Vec<Vec<u8>>, pos: (i32, i32)) -> bool {
    let mut valid = true;

    for (y, row) in layout.iter().enumerate() {
        for (x, block) in row.iter().enumerate() {
            if *block == 1 {
                let x = x as i32 + pos.0;
                let y = y as i32 + pos.1;

                if x < 0 || x >= 10 || y < 0 || y >= 20 || blocks_in_board[y as usize][x as usize] == 1 {
                    valid = false;
                }
            }
        }
    }

    valid
}

