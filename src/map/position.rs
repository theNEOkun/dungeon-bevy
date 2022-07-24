use crate::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn from_tuple((x, y): (f32, f32)) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn new_from_usize(x: usize, y: usize) -> Self {
        let x = x as f32;
        let y = y as f32;
        Self {
            x,
            y
        }
    }

    pub fn from_position(pos: PositionI) -> Self {
        Self {
            x: pos.x as f32,
            y: pos.y as f32
        }
    }

    pub fn from_transform(pos: Transform) -> Self {
        Self {
            x: pos.translation.x,
            y: pos.translation.y,
        }
    }

    pub fn from_index(pos: usize) -> Self {
        let x = pos % (SCREEN_WIDTH as usize);
        let y = pos / (SCREEN_WIDTH as usize);
        Self {
            x: x as f32,
            y: y as f32
        }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn intersect(&self, other: &Self) -> bool {
        self.x <= other.x && self.x > (other.x - 1.0) &&
        self.y <= other.y && self.y > (other.y - 1.0)
    }

    pub fn as_tuple(&self) -> (&f32, &f32) {
        (&self.x, &self.y)
    }

    pub fn integer_position(&self) -> (i32, i32) {
        (self.x.round() as i32, self.y.round() as i32)
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }

    pub fn normalize(self) -> Self {
        let length = f32::sqrt((self.x * self.x) + (self.y * self.y));
        Self {
            x: self.x / length,
            y: self.y / length
        }
    }
}

impl std::cmp::PartialEq<Position> for Position {
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl std::cmp::PartialEq<Transform> for Position {
    fn eq(&self, other: &Transform) -> bool {
        self.x == other.translation.x && self.y == other.translation.y
    }
}

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl std::ops::Add<(f32, f32)> for Position {
    type Output = Self;

    fn add(self, rhs: (f32, f32)) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1
        }
    }
}

impl std::ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl std::ops::Mul for Position {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

#[derive(Component, PartialEq, Clone, Copy)]
pub struct PositionI {
    pub x: i32,
    pub y: i32,
}

impl PositionI {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn new_from_tuple((x, y): (i32, i32)) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    pub fn intersect(&self, other: &Position) -> bool {
        other.x <= self.x as f32 && other.x > (self.x - 1) as f32 &&
        other.x <= self.y as f32 && other.y > (self.x - 1) as f32
    }
}

impl std::ops::Add for PositionI {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl std::ops::Add<(i32, i32)> for PositionI {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1
        }
    }
}

impl std::ops::Sub for PositionI {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl std::ops::Mul for PositionI {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl std::fmt::Debug for PositionI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}
