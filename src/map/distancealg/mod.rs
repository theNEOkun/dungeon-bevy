use super::Position;
use crate::prelude::*;

use std::cmp::{max, min};

pub enum Distance {
    Pythagoras,
    PythagorasSquared,
    Chebyshev,
}

impl Distance {
    pub fn distance2d(self, start: Position, end: Position) -> f32 {
        let start = (start.x as i32, start.y as i32);
        let end = (end.x as i32, end.y as i32);
        match self {
            Self::Pythagoras => distance2d_pythagoras(start, end),
            Self::PythagorasSquared => distance2d_pythagoras_squared(start, end),
            Self::Chebyshev => distance2d_chebyshev(start, end),
        }
    }

    pub fn distance2d_transform(self, start: Transform, end: Transform) -> f32 {
        let start = (start.translation.x as i32, start.translation.y as i32);
        let end = (end.translation.x as i32, end.translation.y as i32);
        match self {
            Self::Pythagoras => distance2d_pythagoras(start, end),
            Self::PythagorasSquared => distance2d_pythagoras_squared(start, end),
            Self::Chebyshev => distance2d_chebyshev(start, end),
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

fn distance2d_chebyshev(start: (i32, i32), end: (i32, i32)) -> f32 {
    std::cmp::max(
        i32::abs(start.0 - end.0),
        i32::abs(start.1 - end.1),
    ) as f32
}
