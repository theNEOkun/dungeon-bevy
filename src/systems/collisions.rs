use crate::prelude::*;
use bevy::sprite::collide_aabb::collide;

pub fn check_for_collisions(
    mut commands: Commands,
    collider_query: Query<(Entity, &Position, &Transform), With<Collider>>,
    wall_positions: Query<(Entity, &Position, &Transform), With<Wall>>,
    mut pre_move: EventReader<CheckCollision>,
    mut post_move: EventWriter<WantsToMove>,
) {
    for (entity, position, player) in collider_query.iter() {
        for (_, _, wall) in wall_positions.iter() {
            for each in pre_move.iter() {
                let dest = Transform::from_xyz(each.destination.x, each.destination.y, 1.0);
                let collision = collide(
                    wall.translation,
                    wall.scale.truncate(),
                    dest.translation,
                    dest.scale.truncate(),
                );
                if let Some(_) = collision {
                    post_move.send(WantsToMove {
                        entity,
                        destination: *position,
                    })
                } else {
                    post_move.send(WantsToMove {
                        entity,
                        destination: each.destination,
                    })
                }
            }
        }
    }
}
