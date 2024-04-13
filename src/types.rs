use std::ops::{Deref, DerefMut, Add, AddAssign};
use bevy::prelude::Component;

use crate::{components::tetromino::{IndexLayout, ShouldHardDrop}, states::game::ShouldMerge};

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

impl IndexLayout {
    pub fn rotate(&mut self, direction: i8) {
        let idx = **self;
        if idx == 0 && direction == -1 {
            **self = 3;
        } else if idx == 3 && direction == 1 {
            **self = 0;
        } else {
            **self = (idx as i8 + direction) as usize;
        }
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

impl Deref for ShouldHardDrop {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ShouldHardDrop {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
