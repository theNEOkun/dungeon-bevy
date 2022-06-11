use bevy::core::FixedTimestep;
use bevy::sprite::collide_aabb::collide;

use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_system(player_movement)
                .with_system(check_for_collisions.after(player_movement))
                .with_system(make_move.after(check_for_collisions))
                .with_system(camera_follows_player.after(make_move)),
        );
    }
}

pub fn make_move(
    mut player: Query<(Entity, &mut Position, &mut Transform), With<Collider>>,
    mut event_reader: EventReader<WantsToMove>,
) {
    for (entity, mut position, mut transform) in player.iter_mut() {
            for each in event_reader.iter() {
                    position.x = each.destination.x;
                    position.y = each.destination.y;
                    transform.translation.x = position.x;
                    transform.translation.y = position.y;
        }
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<(Entity, &mut Transform, &mut Position), With<Player>>,
    mut event_writer: EventWriter<CheckCollision>,
) {
    for (entity, _, position) in player.iter_mut() {
        let mut delta = (0.0, 0.0);
        if keyboard_input.pressed(KeyCode::Left) {
            delta.0 = -0.1;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            delta.0 = 0.1;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            delta.1 = 0.1;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            delta.1 = -0.1;
        }
        let destination = Position::new(position.x + delta.0, position.y + delta.1);
        if delta != (0.0, 0.0) {
            event_writer.send(CheckCollision {
                entity,
                destination,
            });
        }
    }
}

pub fn camera_follows_player(
    mut cameras: Query<&mut Transform, With<Camera>>,
    players: Query<&Position, With<Player>>,
) {
    for player in players.iter() {
        for mut camera in cameras.iter_mut() {
            camera.translation.x = player.x;
            camera.translation.y = player.y;
        }
    }
}
