use std::ops::{Deref, DerefMut};

use bevy::{prelude::*, sprite::Anchor};

use crate::{
    constants::{BOARD_OUTER_SIZE, BOARD_POSITION, BOARD_BORDER_THICKNESS, TETROMINO_SIZE},
    states::{AppState, game::ShouldMerge},
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

pub fn merge_blocks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut shuold_merge: ResMut<ShouldMerge>,
    next_state: ResMut<NextState<AppState>>,
    mut blocks_in_board: ResMut<BlocksInBoard>,
    mut query: Query<(Entity, &Children, &Position), With<Tetromino>>,
    board: Query<Entity, With<Board>>,
    mut tf_query: Query<(&Position, &mut Transform)>,
) {
    if **shuold_merge {
        let (entt, children, pos_t) = query.single_mut();
        let board = board.single();
        for child in children {
            let (pos_b, mut tf) = tf_query.get_mut(*child).unwrap();
            let pos = pos_b + pos_t;
            tf.translation.x = BOARD_BORDER_THICKNESS + pos.x as f32 * TETROMINO_SIZE.x;
            tf.translation.y = -BOARD_BORDER_THICKNESS - pos.y as f32 * TETROMINO_SIZE.y;
            blocks_in_board[pos.y as usize][pos.x as usize] = 1;
            commands.entity(*child).set_parent(board);
        }
        commands.entity(entt).despawn();
        spawn_tetromino(Tetromino::J, &blocks_in_board, &mut commands, &asset_server, board, next_state);
        **shuold_merge = false;
    }
}

#[derive(Debug, Resource)]
pub struct BlocksInBoard(Vec<Vec<u8>>);

impl BlocksInBoard {
    pub fn new() -> Self {
        Self(vec![vec![0; 10]; 20])
    }

    #[allow(unused)]
    pub fn display(&self) {
        for row in self.iter() {
            for block in row.iter() {
                print!("{} ", block);
            }
            println!();
        }
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
                let pos_x = x as i32 + pos.x;
                let pos_y = y as i32 + pos.y;

                if pos_x < 0
                    || pos_x >= 10
                    || pos_y < 0
                    || pos_y >= 20
                    || blocks_in_board[pos_y as usize][pos_x as usize] == 1
                {
                    return false;
                }
            }
        }
    }

    return true;
}
