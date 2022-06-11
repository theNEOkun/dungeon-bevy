use crate::prelude::*;
use bevy::sprite::collide_aabb::collide;

pub fn check_for_collisions(
    mut commands: Commands,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
    wall_positins: Query<(Entity, &Position), With<Wall>>,
    mut collision_events: EventWriter<CollisionEvent>
) {
    for(collider_entity, transform) in collider_query.iter() {
    }
}
