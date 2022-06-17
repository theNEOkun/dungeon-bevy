use crate::prelude::*;

/// Handles the animation for the attacks themselves
pub fn attack_animation(
    mut commands: Commands,
    mut player: Query<(
        Entity,
        &mut TextureAtlasSprite,
        &mut Animations,
        &mut AnimDirection,
        Option<&mut AttackAnim>,
    )>,
    time: Res<Time>,
) {
    for (entity, mut sprite, mut animated, direction, anim) in player.iter_mut() {
        if let Some(_) = anim {
            let animated = &mut animated.attacking;
            animated.timer.tick(time.delta());
            if animated.timer.finished() {
                if animated.counter == 0 {
                    sprite.custom_size = Some(Vec2::new(2.0, 2.0));
                }
                sprite.index = animated.counter + (*direction as usize) / 4 + animated.offset;
                animated.counter += 1;
                if animated.counter >= animated.length {
                    animated.counter = 0;
                    commands.entity(entity).remove::<AttackAnim>();
                    sprite.custom_size = Some(Vec2::new(1.0, 2.0));
                }
            }
        }
    }
}

pub fn attack(
    mut commands: Commands,
    attacker: Query<(Entity, &Animations, &Weapon, &Transform, &AnimDirection)>,
    victim: Query<(Entity, &Transform), With<Living>>,
) {
    for (_, animation, weapon, position, direction) in attacker.iter() {
        let animation = &animation.attacking;
        if weapon.damage_frames[0] <= animation.counter as i32 && weapon.damage_frames[weapon.damage_frames.len() - 1] >= animation.counter as i32 {
            let direction = match direction {
                AnimDirection::Up => (0.0, 1.0),
                AnimDirection::Down => (0.0, -1.0),
                AnimDirection::Left => (1.0, 0.0),
                AnimDirection::Right => (-1.0, 0.0),
            };
            let mut new_pos = position.translation;
            new_pos.x += direction.0;
            new_pos.y += direction.1;
            for (_, transform) in victim.iter() {
                if transform.translation == new_pos {

                }
            }
            println!("Hello");
        }
    }
}

/// Handles the attacking
pub fn on_attack(
    mut commands: Commands,
    player: Query<Entity, With<Living>>,
    mut event_reader: EventReader<WantsToAttack>,
) {
    for entity in player.iter() {
        for each in event_reader.iter() {
            if entity == each.attacker {
                commands.entity(entity).insert(AttackAnim);
            }
        }
    }
}
