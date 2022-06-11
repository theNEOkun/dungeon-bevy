use super::*;

#[derive(Component)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Position,
}

#[derive(Component)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

#[derive(Component)]
pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity,
}

#[derive(Component)]
pub struct CollisionEvent{
    pub mover: Entity,
    pub collider: Entity,
}
