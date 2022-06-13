use crate::prelude::*;

pub fn check_for_collisions(
    mut player: Query<(Entity, &mut Position, &mut Transform, &mut TextureAtlasSprite), With<Collider>>,
    mut event_reader: EventReader<WantsToMove>,
    map: Res<MapBuilder>,
    time: Res<Time>,
) {
    for (_, mut position, mut transform, mut sprite) in player.iter_mut() {
        for each in event_reader.iter() {
            let destination = Position::new(
                position.x + each.destination.x,
                position.y + each.destination.y,
            );
            println!("{destination:?}");
            if let Some(animated) = &each.animation {
                let mut timer = Timer::from_seconds(animated.timer, true);
                timer.tick(time.delta());
                if timer.finished() {
                    sprite.index = ((sprite.index + 1) % 4) + animated.direction as usize;
                }
            }

                    move_char(&mut position, &mut transform, &map, destination);
        }
    }
}

fn move_char(
    mut position: &mut Position,
    transform: &mut Transform,
    map: &Res<MapBuilder>,
    destination: Position,
) {
    if map.map.can_enter_tile_f(&destination) {
        position.x = destination.x;
        position.y = destination.y;
        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}

pub fn animation(
    mut player: Query<(Entity, &mut TextureAtlasSprite)>,
    mut event_reader: EventReader<WantsToMove>,
    time: Res<Time>,
) {
    for (_, mut sprite) in player.iter_mut() {
        for event in event_reader.iter() {
        }
    }
}
