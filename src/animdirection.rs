use crate::prelude::*;

#[derive(Component, Copy, Clone, PartialEq, Debug)]
pub enum AnimDirection {
    Up = 16 * 2,
    Down = 16 * 0,
    Left = 16 * 1,
    Right = 16 * 3,
}

impl AnimDirection {
    pub fn get_direction(&self) -> (f32, f32) {
        match self {
            AnimDirection::Up => (0.0, 1.0),
            AnimDirection::Down => (0.0, -1.0),
            AnimDirection::Left => (1.0, 0.0),
            AnimDirection::Right => (-1.0, 0.0),
        }
    }

    pub fn match_direction(direction: (f32, f32)) -> Self {
        if direction.0 != 0.0 {
            if direction.0 < 0.0 {
                AnimDirection::Right
            } else {
                AnimDirection::Left
            }
        } else if direction.1 != 0.0 {
            if direction.1 < 0.0 {
                AnimDirection::Down
            } else {
                AnimDirection::Up
            }
        } else {
            AnimDirection::Down
        }
    }

    pub fn match_position(direction: Position) -> Self {
        if direction.x != 0.0 {
            if direction.x < 0.0 {
                AnimDirection::Right
            } else {
                AnimDirection::Left
            }
        } else if direction.y != 0.0 {
            if direction.y < 0.0 {
                AnimDirection::Down
            } else {
                AnimDirection::Up
            }
        } else {
            AnimDirection::Down
        }
    }

    pub fn match_position_prev(direction: Position, prev_dirr: Self) -> Self {
        if direction.x != 0.0 {
            if direction.x < 0.0 {
                AnimDirection::Right
            } else {
                AnimDirection::Left
            }
        } else if direction.y != 0.0 {
            if direction.y < 0.0 {
                AnimDirection::Down
            } else {
                AnimDirection::Up
            }
        } else {
            prev_dirr
        }
    }
}
