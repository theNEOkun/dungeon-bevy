use crate::prelude::*;

pub fn check_for_collisions(
    mut player: Query<
        (Entity, &mut Transform, &mut Living),
        (With<TextureAtlasSprite>, With<AnimDirection>),
    >,
    mut event_reader: EventReader<WantsToMove>,
    map: Res<MapBuilder>,
    time: Res<Time>,
) {
    for each in event_reader.iter() {
        for (entity, mut transform, living) in player.iter_mut() {
            let test_transform = *transform;
            if each.entity == entity {
                let goalx = transform.translation.x + each.destination.x;
                let goaly = transform.translation.y + each.destination.y;
                if map.map.in_bounds_tuple((goalx, goaly)) {
                    transform.translation.x += each.destination.x * living.speed * time.delta_seconds();
                    transform.translation.y += each.destination.y * living.speed * time.delta_seconds();
                }
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
    for each in event_reader.iter() {
        for (entity, mut sprite, mut animated, direction) in player.iter_mut() {
            if each.entity == entity {
                sprite.custom_size = Some(Vec2::new(1.0, 2.0));
                let animated = &mut animated.walking;
                animated.timer.tick(time.delta());
                if animated.timer.finished() {
                    sprite.index = ((sprite.index + 1) % animated.length)
                        + *direction as usize
                        + animated.offset;
                }
            }
        }
    }
}

pub fn chasing(
    mut commands: Commands,
    enemies: Query<(Entity, &mut Transform), (With<Enemy>, With<ChasingPlayer>, Without<Attacked>)>,
    mut player: Query<(Entity, &mut Transform), (With<Player>, Without<Enemy>)>,
    mut event_writer_attack: EventWriter<WantsToAttack>,
    map: Res<MapBuilder>,
) {
    for (e_entity, e_position) in enemies.iter() {
        let (_, target) = player.single_mut();

        let map = &map.map;

        let e_index = map::trans_to_index(*e_position);
        let e_target = map::trans_to_index(*target);
 
        let result = astar(
            &e_index,
            |p| map.get_available_exits(*p),
            |p| map.get_pathing_distance(*p, e_target) as u32,
            |p| e_target == *p,
        );

        if let Some(result) = result {
            if result.1 > 1 {
                let move_to = Position::from_index(result.0[1]);
                if result.0[1] == e_target {
                    event_writer_attack.send(
                        WantsToAttack {
                            attacker: e_entity
                        }
                    )
                } else {
                let destination = move_to - *e_position;
                commands
                    .entity(e_entity)
                    .insert(Movement { goal: destination.normalize() });
                }
            }
        } else {
            commands
                .entity(e_entity)
                .remove::<Movement>();
        }
    }
}

pub fn make_move(
    enemies: Query<(Entity, &mut Movement)>,
    mut event_writer: EventWriter<WantsToMove>,
) {
    for (e_entity, destination) in enemies.iter() {
        let destination = destination.goal;
        event_writer.send(WantsToMove {
            entity: e_entity,
            destination,
        })
    }
}
