use std::collections::HashSet;
use super::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Living {
    pub speed: f32,
    pub current_hp: u8,
    pub max_hp: u8
}

impl Living {
    /// Checks if hp is <= 0
    pub fn is_dead(&self) -> bool {
        self.current_hp <= 0
    }
}

#[derive(Component)]
pub struct Attacked {
    pub damage: u8,
}

#[derive(Component)]
pub struct Animated {
    pub timer: Timer,
    pub offset: usize,
    pub length: usize,
    pub counter: usize,
}

#[derive(Component)]
pub struct Animations {
    pub walking: Animated,
    pub attacking: Animated,
}

#[derive(Component)]
pub struct AttackAnim;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

/// Components to the entity
#[derive(Component)]
pub struct MapLevel(pub i32);

#[derive(Component)]
pub struct MovingRandomly;

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
