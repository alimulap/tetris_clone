use bevy::prelude::Component;
use std::ops::{Add, AddAssign, Deref, DerefMut};

use crate::screens::ingame::ShouldMerge;

#[derive(Debug, Clone, Copy, Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn add_some_cloned(&mut self, other: &[&Position]) -> Self {
        let mut result = *self;
        for p in other.iter() {
            result += **p;
        }
        result
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

impl<'a> Add<&'a Position> for &'a Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Deref for ShouldMerge {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ShouldMerge {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
