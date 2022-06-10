use super::*;

#[derive(Component)]
pub struct Item(pub u64);

#[derive(Component)]
pub struct AmuletOfYala;

#[derive(Component)]
pub struct ProvidesHealing {
    pub amount: i32,
}

#[derive(Component)]
pub struct ProvidesDungeonMap;

#[derive(Component)]
pub struct Carried(pub Entity);

#[derive(Component)]
pub struct Weapon;
