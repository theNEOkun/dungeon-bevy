mod position;
mod distancealg;

use crate::prelude::*;
pub use position::*;
pub use distancealg::Distance;

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

pub struct Error;

type Result<T> = std::result::Result<T, Error>;

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

pub fn trans_to_index(trans: Transform) -> usize {
    ((trans.translation.y.round() * SCREEN_WIDTH) + trans.translation.x.round()) as usize
}

/// Turns x and y into index
///
/// applies map_idx on a new point
pub fn map_idx_int(x: i32, y: i32) -> usize {
    map_idx(&PositionI::new(x, y))
}

pub fn idx_to_pos(pos: usize) -> Position {
    Position::from_index(pos)
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

    /// Checks if a person can enter that point
    ///
    /// @returns true if it is enterable
    pub fn can_enter_tile_idx(&self, point: usize) -> bool {
        self.in_bounds_idx(point) && (
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

    pub fn in_bounds_trans(&self, point: &Transform) -> bool {
        point.translation.x >= 0.0 && point.translation.x < SCREEN_WIDTH && point.translation.y >= 0.0 && point.translation.y < SCREEN_HEIGHT

    }

    pub fn in_bounds_tuple(&self, point: (f32, f32)) -> bool {
        point.0 >= 0.0 && point.0 < SCREEN_WIDTH && point.1 >= 0.0 && point.1 < SCREEN_HEIGHT
    }

    /// Checks to see if a point is in bounds
    /// WIDTH > x >= 0 and HEIGHT > y >= 0
    ///
    /// @param point is the point to check 
    /// @returns true if it is
    pub fn in_bounds_f(&self, point: &Position) -> bool {
        point.x >= 0.0 && point.x < SCREEN_WIDTH  && point.y >= 0.0 && point.y < SCREEN_HEIGHT
    }

    /// Checks to see if a point is in bounds
    /// WIDTH > x >= 0 and HEIGHT > y >= 0
    ///
    /// @param point is the point to check 
    /// @returns true if it is
    pub fn in_bounds_idx(&self, point: usize) -> bool {
        point > 0 && point < (SCREEN_WIDTH * SCREEN_HEIGHT) as usize
    }

    /// Checks t osee if a position is a valied exit
    ///
    /// @param loc is the current position
    /// @param delta is the position to walk to
    /// @return Either Some(index) else None
    fn valid_exit(&self, loc: Position, delta: Position) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds_f(&destination) {
            if self.can_enter_tile_f(&destination) {
                let idx = destination.to_index();
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn point_to_index(&self, pos: &Position) -> usize {
        ((pos.y * SCREEN_WIDTH) + pos.x) as usize
    }

    pub fn pos_to_index(&self, x: f32, y: f32) -> usize {
        ((y * SCREEN_WIDTH) + x) as usize
    }
    
    /// used to get the neighbours of a given cell
    ///
    /// @param position is the position to get the neighbours of
    /// @return a vec of results of all the neighbours position
    pub fn get_neighbours(&self, position: usize) -> Vec<(usize, u32)> {
        let mut arr: Vec<(usize, u32)> = Vec::new();

        let position = Position::from_index(position);

        let test_x = position.x;
        let test_y = position.y;

        if (test_x + 1.0) < SCREEN_HEIGHT {
            let idx = self.pos_to_index(test_x + 1.0, test_y);
            arr.push((idx, 1));
        }
        if (test_x - 1.0) > 0.0 {
            let idx = self.pos_to_index(test_x - 1.0, test_y);
            arr.push((idx, 1));
        }
        if (test_y + 1.0) < SCREEN_WIDTH {
            let idx = self.pos_to_index(test_x, test_y + 1.0);
            arr.push((idx, 1));
        }
        if (test_y - 1.0) > 0.0 {
            let idx = self.pos_to_index(test_x, test_y - 1.0);
            arr.push((idx, 1));
        }

        arr
    }

    pub fn get_available_exits(&self, idx: usize) -> Vec<(usize, u32)> {
        let mut exits = Vec::new();
        let location = Position::from_index(idx);

        if let Some(idx) = self.valid_exit(location, Position::new(-1.0, 0.0)) {
            exits.push((idx, 1))
        }
        if let Some(idx) = self.valid_exit(location, Position::new(1.0, 0.0)) {
            exits.push((idx, 1))
        }
        if let Some(idx) = self.valid_exit(location, Position::new(0.0, -1.0)) {
            exits.push((idx, 1))
        }
        if let Some(idx) = self.valid_exit(location, Position::new(0.0, 1.0)) {
            exits.push((idx, 1))
        }

        exits
    }

    pub fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        Distance::Chebyshev.distance2d(
            Position::from_index(idx1),
            Position::from_index(idx2)
        )
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
