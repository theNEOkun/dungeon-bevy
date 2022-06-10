use crate::prelude::*;

#[derive(Component, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position {
            x,
            y
        }
    }

    pub fn zero() -> Self {
        Position::new(0, 0)
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
