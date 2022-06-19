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
) {
    let shape = Collider::ball(0.25);
    let shape_rot = 0.0;
    let groups = InteractionGroups::all();
    let filter = None;

    for (entity, animation, weapon, position, direction) in attacker.iter() {
        let animation = &animation.attacking;
        if weapon.damage_frames[0] <= animation.counter as i32
            && weapon.damage_frames[weapon.damage_frames.len() - 1] >= animation.counter as i32
        {
            let mut new_pos = position.translation;
            let direction = direction.get_direction();
            new_pos.x += direction.0 * 0.9;
            new_pos.y += direction.1 * 0.9;
            let entity = commands.entity(entity).id();

            rapier_context.intersections_with_shape(
                new_pos.truncate(),
                shape_rot,
                &shape,
                groups,
                filter,
                |victim| {
                    if victim != entity {
                        commands.entity(victim).insert(Attacked {
                            damage: weapon.damage,
                        });
                    }
                    true
                },
            );
        }
    }
}

pub fn after_attack(
    mut commands: Commands,
    mut targets: Query<(Entity, &mut Attacked, &mut Living)>,
    mut event: EventReader<Attack>,
) {
    for (entity, attack, mut hp) in targets.iter_mut() {
        println!("Entity {entity:?}");
        for _ in event.iter() {
            hp.current_hp -= attack.damage;
            println!("{} {}", hp.max_hp, hp.current_hp);
            if hp.is_dead() {
                commands.entity(entity).despawn_recursive();
            }
            commands.entity(entity).remove::<Attacked>();
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
