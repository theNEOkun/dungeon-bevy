use crate::prelude::*;

pub fn attack_animation(
    mut player: Query<(
        Entity,
        &mut TextureAtlasSprite,
        &mut Animations,
        &mut AnimDirection,
    )>,
    mut event_reader: EventReader<WantsToAttack>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut animated, direction) in player.iter_mut() {
        for event in event_reader.iter() {
            if event.attacker != entity {
                return;
            }
            let animated = &mut animated.attacking;
            animated.timer.tick(time.delta());
            if animated.timer.finished() {
                sprite.index =
                    0 + *direction as usize + animated.offset;
                animated.counter += 1;
                if animated.counter >= animated.length {
                    animated.counter = 0;
                }
            }
        }
    }
}
