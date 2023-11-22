use std::ops::{Deref, DerefMut, Add, AddAssign};
use bevy::prelude::Component;

use crate::components::tetromino::IndexLayout;

#[derive(Clone, Copy, Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
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

