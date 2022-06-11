use crate::prelude::*;
use bevy::sprite::collide_aabb::collide;

pub fn check_for_collisions(
    mut commands: Commands,
    collider_query: Query<(Entity, &Position, &Transform), With<Collider>>,
    wall_positions: Query<(Entity, &Position), With<Wall>>,
    mut collision_events: EventWriter<CollisionEvent>
) {
    for(_, e_position, _) in collider_query.iter() {
        for (_, posits) in wall_positions.iter() {
            println!("{e_position:?}, {posits:?}");
            if posits.intersect(e_position) {
                collision_events.send(CollisionEvent);
            }
        }
    }
}
