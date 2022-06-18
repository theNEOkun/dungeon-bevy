use crate::prelude::*;

pub fn check_for_collisions(
    mut player: Query<(Entity, &mut Transform, &mut Living)>,
    mut event_reader: EventReader<WantsToMove>,
    time: Res<Time>,
) {
    for (entity, mut transform, living) in player.iter_mut() {
        for each in event_reader.iter() {
            if each.entity == entity {
                transform.translation.x += each.destination.x * living.speed * time.delta_seconds();
                transform.translation.y += each.destination.y * living.speed * time.delta_seconds();
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
            sprite.custom_size = Some(Vec2::new(1.0, 2.0));
            let animated = &mut animated.walking;
            animated.timer.tick(time.delta());
            if animated.timer.finished() {
                sprite.index =
                    ((sprite.index + 1) % animated.length) + *direction as usize + animated.offset;
            }
        }
    }
}
