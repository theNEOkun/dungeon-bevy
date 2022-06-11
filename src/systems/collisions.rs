use crate::prelude::*;
use bevy::sprite::collide_aabb::collide;

pub fn check_for_collisions(
    mut commands: Commands,
    collider_query: Query<(Entity, &Position, &Transform), With<Collider>>,
    wall_positions: Query<(Entity, &Position, &Transform), With<Wall>>,
    mut collision_events: EventWriter<CollisionEvent>
) {
    for (mover, mover_position, player) in collider_query.iter() {
        for (collider, collider_position, wall) in wall_positions.iter() {
            let collision = collide(
                wall.translation,
                wall.scale.truncate(),
                player.translation,
                player.scale.truncate()
            );
            if let Some(_) = collision {
                collision_events.send(CollisionEvent{
                    mover,
                    collider,
                    fix: (0.0, 0.0)
                });
            }
        }
    }
}
