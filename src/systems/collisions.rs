use crate::prelude::*;
use bevy::sprite::collide_aabb::collide;

pub fn check_for_collisions(
    mut commands: Commands,
    collider_query: Query<(Entity, &Position, &Transform), With<Collider>>,
    wall_positions: Query<(Entity, &Position, &Transform), With<Wall>>,
    mut pre_move: EventReader<CheckCollision>,
    mut post_move: EventWriter<WantsToMove>,
) {
    for each in pre_move.iter() {
        for (_, _, player) in collider_query.iter() {
            for (_, _, wall) in wall_positions.iter() {
                let collision = collide(
                    wall.translation,
                    wall.scale.truncate(),
                    player.translation,
                    player.scale.truncate(),
                );
                if let Some(_) = collision {
                    post_move.send(WantsToMove {
                        entity: each.entity,
                        destination: Position::zero(),
                        direction: AnimDirection::Down
                    })
                } else {
                    post_move.send(WantsToMove {
                        entity: each.entity,
                        destination: each.destination,
                        direction: AnimDirection::Down
                    })
                }
            }
        }
    }
}
