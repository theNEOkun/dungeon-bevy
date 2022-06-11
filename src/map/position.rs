use crate::prelude::*;

#[derive(Component, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position {
            x,
            y
        }
    }

    pub fn new_from_tuple((x, y): (f32, f32)) -> Self {
        Position {
            x,
            y
        }
    }

    pub fn zero() -> Self {
        Position::new(0.0, 0.0)
    }
}

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl std::ops::Add<(f32, f32)> for Position {
    type Output = Self;

    fn add(self, rhs: (f32, f32)) -> Self::Output {
        Position {
            x: self.x + rhs.0,
            y: self.y + rhs.1
        }
    }
}

impl std::ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl std::ops::Mul for Position {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}
