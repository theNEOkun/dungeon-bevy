use crate::prelude::*;

pub fn check_for_collisions(
    mut player: Query<(Entity, &mut Transform, &mut Living), (With<TextureAtlasSprite>, With<AnimDirection>)>,
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
    for (entity, mut sprite, mut animated, direction) in player.iter_mut() {
        for each in event_reader.iter() {
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

fn point_to_index(trans: Transform) -> usize {
    ((trans.translation.y / SCREEN_WIDTH) + trans.translation.x % SCREEN_WIDTH).round() as usize
}

pub fn chasing(
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut Transform), With<Enemy>>,
    mut player: Query<(Entity, &mut Transform), With<Player>>,
    map: Res<MapBuilder>,
) {
    for (_, e_position) in enemies.iter() {
        let (_, target) = player.single_mut();

        let map = &map.map;
        let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize, &[point_to_index(*target)], map, 1024.0);

        let idx = point_to_index(*e_position);

        if let Some(destination)= DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {

        }
    }
}
