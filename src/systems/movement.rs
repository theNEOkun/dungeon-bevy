use crate::prelude::*;

pub fn check_for_collisions(
    mut player: Query<
        (
            Entity,
            &mut Position,
            &mut Transform,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
        ),
        With<Collider>,
    >,
    walls: Query<(Entity, &Position, &Transform), (With<Wall>, Without<Collider>)>,
    mut event_reader: EventReader<WantsToMove>,
    map: Res<Map>,
    time: Res<Time>,
) {
    for (_, mut position, mut transform, mut timer, mut sprite) in player.iter_mut() {
        for (_, _, _) in walls.iter() {
            for each in event_reader.iter() {
                let destination = Position::new(
                    position.x + each.destination.x,
                    position.y + each.destination.y,
                );
                let can_move = map.can_enter_tile_f(&destination);
                if can_move {
                    timer.tick(time.delta());
                    if timer.just_finished() {
                        sprite.index = ((sprite.index + 1) % 4) + each.direction as usize;
                        position.x = destination.x;
                        position.y = destination.y;
                        transform.translation.x = position.x;
                        transform.translation.y = position.y;
                    }
                }
            }
        }
    }
}
