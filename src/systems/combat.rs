use crate::prelude::*;

pub fn attack_animation(
    mut player: Query<(
        Entity,
        &mut TextureAtlasSprite,
        &mut Animated,
        &mut AnimDirection,
    )>,
    mut event_reader: EventReader<WantsToAttack>,
    time: Res<Time>,
) {
    for (_, mut sprite, mut animated, direction) in player.iter_mut() {
        for _ in event_reader.iter() {
            animated.timer.tick(time.delta());
            if animated.timer.finished() {
                sprite.index = ((sprite.index + 1) % animated.length) + *direction as usize + animated.offset;
            }
        }
    }
}
