pub mod entities;
pub mod items;
pub mod messages;

pub use crate::prelude::*;
pub use bevy::ecs::component::Component;
pub use bevy::ecs::entity::Entity;

pub use entities::*;
pub use items::*;
pub use messages::*;

/// Rendering

#[derive(Component)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: char,
}

#[derive(Component)]
pub struct RenderDual {
    pub color: ColorPair,
    pub glyph: (char, char),
}

#[derive(Component, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position {
            x, y
        }
    }

    pub fn zero() -> Self {
        Position {
            x: 0,
            y: 0
        }
    }
}

impl std::ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Name(pub String);
