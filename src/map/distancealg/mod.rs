use super::Position;

use std::cmp::{max, min};

pub enum Distance {
    Pythagoras,
    PythagorasSquared,
}

impl Distance {
    pub fn distance2d(self, start: Position, end: Position) -> f32 {
        let start = (start.x as i32, start.y as i32);
        let end = (end.x as i32, end.y as i32);
        match self {
            Self::Pythagoras => distance2d_pythagoras(start, end),
            Self::PythagorasSquared => distance2d_pythagoras_squared(start, end),
        }
    }
}

fn distance2d_pythagoras_squared(start: (i32, i32), end: (i32, i32)) -> f32 {
    let dx = (max(start.0 as i32, end.0 as i32) - min(start.0, end.0)) as f32;
    let dy = (max(start.1 as i32, end.1 as i32) - min(start.1, end.1)) as f32;
    (dx * dx) + (dy * dy)
}

fn distance2d_pythagoras(start: (i32, i32), end: (i32, i32)) -> f32 {
    let result = distance2d_pythagoras_squared(start, end);
    f32::sqrt(result)
}
