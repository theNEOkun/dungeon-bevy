use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    pub fn with_size(x1: i32, y1: i32, width: i32, height: i32) -> Self {
        Self {
            x1,
            y1,
            x2: x1 + width,
            y2: y1 + height,
        }
    }

    /// Used to find the center of a Rect
    pub fn center(&self) -> Position {
        Position::new(((self.x1 + self.x2) / 2) as f32, ((self.y1 + self.y2) / 2) as f32)
    }

    pub fn center_int(&self) -> PositionI {
        PositionI::new((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    /// Used to check if another rect intersects with this one
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(PositionI),
    {
        for y in self.y1..=self.y2 {
            for x in self.x1..=self.x2 {
                f(PositionI::new(x, y))
            }
        }
    }
}

impl std::fmt::Debug for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(x1: {}, y1: {}), (x1: {}, y2: {})",
            self.x1, self.y1, self.x2, self.y2
        )
    }
}
