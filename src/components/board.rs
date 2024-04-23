use std::ops::{Deref, DerefMut};

use bevy::{prelude::*, sprite::Anchor};

use crate::{
    constants::{BOARD_BORDER_THICKNESS, BOARD_OUTER_SIZE, BOARD_POSITION, TETROMINO_SIZE},
    states::{
        game::{InGameAssets, ShouldMerge},
        AppState,
    },
    types::Position,
};

use super::tetromino::{spawn_tetromino, Tetromino};

#[derive(Component)]
pub struct Board;

pub fn setup(
    mut commands: Commands,
    ingame_assets: Res<InGameAssets>,
    blocks_in_board: Res<BlocksInBoard>,
    next_state: ResMut<NextState<AppState>>,
) {
    let board = commands
        .spawn((
            Board,
            SpriteBundle {
                texture: ingame_assets.board_tex.clone(),
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
        [
            Tetromino::I,
            Tetromino::O,
            Tetromino::T,
            Tetromino::S,
            Tetromino::Z,
            Tetromino::J,
            Tetromino::L,
        ][fastrand::usize(0..7)],
        &blocks_in_board,
        &mut commands,
        &ingame_assets.tetromino_tex,
        board,
        next_state,
    );
}

#[allow(clippy::too_many_arguments)]
pub fn merge_blocks(
    mut commands: Commands,
    ingame_assets: Res<InGameAssets>,
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
        commands.entity(board).remove_children(&[entt]);
        commands.entity(entt).despawn();
        spawn_tetromino(
            [
                Tetromino::I,
                Tetromino::O,
                Tetromino::T,
                Tetromino::S,
                Tetromino::Z,
                Tetromino::J,
                Tetromino::L,
            ][fastrand::usize(0..7)],
            &blocks_in_board,
            &mut commands,
            &ingame_assets.tetromino_tex,
            board,
            next_state,
        );
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
    layout: &[Vec<u8>],
    pos: &Position,
) -> bool {
    for (y, row) in layout.iter().enumerate() {
        for (x, block) in row.iter().enumerate() {
            if *block == 1 {
                let pos_x = x as i32 + pos.x;
                let pos_y = y as i32 + pos.y;

                if !(0..10).contains(&pos_x)
                    || !(0..20).contains(&pos_y)
                    || blocks_in_board[pos_y as usize][pos_x as usize] == 1
                {
                    return false;
                }
            }
        }
    }

    true
}
