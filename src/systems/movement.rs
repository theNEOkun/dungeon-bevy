use crate::prelude::*;

pub fn check_for_collisions(
    mut player: Query<(Entity, &mut Position, &mut Transform), With<Collider>>,
    mut event_reader: EventReader<WantsToMove>,
    map: Res<MapBuilder>,
    time: Res<Time>,
) {
    for (_, mut position, mut transform) in player.iter_mut() {
        for each in event_reader.iter() {
            let destination = Position::new(
                position.x + each.destination.x * (time.delta_seconds() * 10.0),
                position.y + each.destination.y * (time.delta_seconds() * 10.0),
            );
            if map.map.can_enter_tile_f(&destination) {
                position.x = destination.x;
                position.y = destination.y;
                transform.translation.x = position.x;
                transform.translation.y = position.y;
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
