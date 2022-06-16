use crate::prelude::*;

pub fn check_for_collisions(
    mut player: Query<(Entity, &mut Transform), With<Living>>,
    mut event_reader: EventReader<WantsToMove>,
    time: Res<Time>,
) {
    for (entity, mut transform) in player.iter_mut() {
        for each in event_reader.iter() {
            if each.entity == entity {
                transform.translation.x += each.destination.x * time.delta_seconds();
                transform.translation.y += each.destination.y * time.delta_seconds();
            }
        }
    }
}

pub fn walking_animation(
    mut player: Query<(
        Entity,
        &mut TextureAtlasSprite,
        &mut Animations,
        &mut AnimDirection,
    )>,
    mut event_reader: EventReader<WantsToMove>,
    time: Res<Time>,
) {
    for (_, mut sprite, mut animated, direction) in player.iter_mut() {
        for _ in event_reader.iter() {
            let animated = &mut animated.walking;
            animated.timer.tick(time.delta());
            if animated.timer.finished() {
                sprite.index =
                    ((sprite.index + 1) % animated.length) + *direction as usize + animated.offset;
            }
        }
    }
}
