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
    mut event: EventWriter<Attack>,
) {
    for (entity, mut sprite, mut animated, direction, anim) in player.iter_mut() {
        if let Some(_) = anim {
            let animated = &mut animated.attacking;
            animated.timer.tick(time.delta());
            if animated.timer.finished() {
                sprite.custom_size = Some(Vec2::new(2.0, 2.0));
                sprite.index = animated.counter + (*direction as usize) / 4 + animated.offset;
                animated.counter += 1;
                if animated.counter >= animated.length {
                    animated.counter = 0;
                    commands.entity(entity).remove::<AttackAnim>();
                    event.send(Attack {});
                    sprite.custom_size = Some(Vec2::new(1.0, 2.0));
                }
            }
        }
    }
}

pub fn attack(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    attacker: Query<(Entity, &Animations, &Weapon, &Transform, &AnimDirection)>,
    victim: Query<(Entity, &Transform), With<Living>>,
) {
    for (_, animation, weapon, position, direction) in attacker.iter() {
        let animation = &animation.attacking;
        if weapon.damage_frames[0] <= animation.counter as i32 && weapon.damage_frames[weapon.damage_frames.len() - 1] >= animation.counter as i32 {
            let mut new_pos = position.translation;
            let direction = direction.get_direction();
            new_pos.x += direction.0;
            new_pos.y += direction.1;
            for (enemy, transform) in victim.iter() {
                if transform.translation.x as i32 == new_pos.x as i32 && transform.translation.y as i32 == new_pos.y as i32 {
                    println!("{:?} == {new_pos:?}", transform.translation);
                    commands.entity(enemy).insert(Attacked {
                        damage: weapon.damage,
                    });
                }
            }
        }
    }
}

pub fn after_attack(
    mut commands: Commands,
    mut targets: Query<(Entity, &mut Attacked, &mut Living)>,
    mut event: EventReader<Attack>,
) {
    for (entity, attack, mut hp) in targets.iter_mut() {
        for _ in event.iter() {
            hp.current_hp -= attack.damage;
            if hp.is_dead() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

/// Handles the attacking
pub fn on_attack(
    mut commands: Commands,
    player: Query<Entity, (With<Living>, With<TextureAtlasSprite>, With<Animations>)>,
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
