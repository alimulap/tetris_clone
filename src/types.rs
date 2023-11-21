use std::ops::{Deref, DerefMut};
use bevy::prelude::Component;

use crate::tetromino::IndexLayout;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Deref for IndexLayout {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for IndexLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

