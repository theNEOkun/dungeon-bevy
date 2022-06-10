use std::collections::HashSet;
use super::*;

#[derive(Component)]
pub struct Player(pub u64);

#[derive(Component)]
pub struct Enemy(pub u64);

/// Components to the entity
#[derive(Component)]
pub struct MapLevel(pub i32);

#[derive(Component)]
pub struct MovingRandomly;

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct ChasingPlayer;

#[derive(Component)]
pub struct Damage(pub i32);

#[derive(Component)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Position>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}
