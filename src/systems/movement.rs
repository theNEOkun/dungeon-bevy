use crate::prelude::*;

pub fn check_for_collisions(
    mut player: Query<(Entity, &mut Position, &mut Transform), With<Collider>>,
    walls: Query<(Entity, &Position, &Transform), (With<Wall>, Without<Collider>)>,
    mut event_reader: EventReader<WantsToMove>,
    map: Res<MapBuilder>,
) {
    for (_, mut position, mut transform) in player.iter_mut() {
        for (_, _, _) in walls.iter() {
            for each in event_reader.iter() {
                let destination = Position::new(
                    position.x + each.destination.x,
                    position.y + each.destination.y,
                );
                let can_move = map.map.can_enter_tile_f(&destination);
                if can_move {
                    position.x = destination.x;
                    position.y = destination.y;
                    transform.translation.x = position.x;
                    transform.translation.y = position.y;
                }
            }
        }
    }
}

pub fn animation(
    mut player: Query<(Entity, &mut TextureAtlasSprite, &mut Animated)>,
    mut event_reader: EventReader<WantsToMove>,
) {
    for (_, mut sprite, animated) in player.iter_mut() {
        for _ in event_reader.iter() {
            sprite.index = ((sprite.index + 1) % 4) + animated.direction as usize;
        }
    }
}
