mod position;

use crate::prelude::*;
pub use position::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    Exit,
}

impl std::fmt::Debug for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Holds the Map
pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
}

/// Moves from a point into an index
///
/// point.y * WIDTH + point.x
pub fn map_idx(point: &PositionI) -> usize {
    ((point.y * SCREEN_WIDTH as i32) + point.x) as usize
}

/// Moves from a point into an index
///
/// point.y * WIDTH + point.x
pub fn map_idx_f(point: &Position) -> usize {
    ((point.y as i32 * SCREEN_WIDTH as i32) + point.x as i32) as usize
}

/// Turns x and y into index
///
/// applies map_idx on a new point
pub fn map_idx_int(x: i32, y: i32) -> usize {
    map_idx(&PositionI::new(x, y))
}

impl Map {
    /// Creates a new Map, with all floor-tiles and false revealed_tiles
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            revealed_tiles: vec![false; NUM_TILES],
        }
    }

    /// Tries to see if a point is inside the map
    ///
    /// @param point is the point to check
    /// @returns None if it is not, else Some(point)
    pub fn try_idx(&self, point: PositionI) -> Option<PositionI> {
        if !self.in_bounds(&point) {
            None
        } else {
            Some(point)
        }
    }

    /// Tries to see if a point is inside the map
    ///
    /// @param point is the point to check
    /// @returns None if it is not, else Some(point)
    pub fn try_idx_f(&self, point: Position) -> Option<Position> {
        if !self.in_bounds_f(&point) {
            None
        } else {
            Some(point)
        }
    }

    /// Checks if a person can enter that point
    ///
    /// @returns true if it is enterable
    pub fn can_enter_tile(&self, point: &PositionI) -> bool {
        self.in_bounds(point) && (
            self[point] == TileType::Floor ||
            self[point] == TileType::Exit
        )
    }

    /// Checks if a person can enter that point
    ///
    /// @returns true if it is enterable
    pub fn can_enter_tile_f(&self, point: &Position) -> bool {
        self.in_bounds_f(point) && (
            self[point] == TileType::Floor ||
            self[point] == TileType::Exit
        )
    }

    /// Checks to see if a point is in bounds
    /// WIDTH > x >= 0 and HEIGHT > y >= 0
    ///
    /// @param point is the point to check 
    /// @returns true if it is
    pub fn in_bounds(&self, point: &PositionI) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH as i32 && point.y >= 0 && point.y < SCREEN_HEIGHT as i32
    }

    /// Checks to see if a point is in bounds
    /// WIDTH > x >= 0 and HEIGHT > y >= 0
    ///
    /// @param point is the point to check 
    /// @returns true if it is
    pub fn in_bounds_f(&self, point: &Position) -> bool {
        point.x >= 0.0 && point.x < SCREEN_WIDTH  && point.y >= 0.0 && point.y < SCREEN_HEIGHT
    }

    /// Checks t osee if a position is a valied exit
    ///
    /// @param loc is the current position
    /// @param delta is the position to walk to
    /// @return Either Some(index) else None
    fn valid_exit(&self, loc: PositionI, delta: PositionI) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(&destination) {
            if self.can_enter_tile(&destination) {
                let idx = self.point_to_index(&destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn point_to_index(&self, pos: &PositionI) -> usize {
        ((pos.y * SCREEN_WIDTH as i32) + pos.x) as usize
    }

    pub fn index_to_point(&self, index: usize) -> PositionI {
        let x = index % SCREEN_WIDTH as usize;
        let y = index / SCREEN_WIDTH as usize;
        PositionI::new(x as i32, y as i32)
    }
}

// Indexing below

impl std::ops::Index<&PositionI> for Map {
    type Output = TileType;

    fn index(&self, point: &PositionI) -> &Self::Output {
        let idx = map_idx(point);
        &self.tiles[idx]
    }
}

impl std::ops::IndexMut<&PositionI> for Map {
    fn index_mut(&mut self, point: &PositionI) -> &mut Self::Output {
        let idx = map_idx(point);
        &mut self.tiles[idx]
    }
}
impl std::ops::Index<&Position> for Map {
    type Output = TileType;

    fn index(&self, point: &Position) -> &Self::Output {
        let idx = map_idx_f(point);
        println!("{idx}");
        &self.tiles[idx]
    }
}

impl std::ops::IndexMut<&Position> for Map {
    fn index_mut(&mut self, point: &Position) -> &mut Self::Output {
        let idx = map_idx_f(point);
        &mut self.tiles[idx]
    }
}

impl std::ops::Index<usize> for Map {
    type Output = TileType;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tiles[index]
    }
}

impl std::ops::IndexMut<usize> for Map {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.tiles[index]
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.tiles)
    }
}
