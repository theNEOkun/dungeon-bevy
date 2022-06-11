use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(player_movement.before(check_for_collisions))
            .add_system(camera_follows_player.after(player_movement));
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<(Entity, &mut Transform, &mut Position), With<Player>>,
    mut collision: EventReader<CollisionEvent>,
) {
    for (entity, mut transform, mut position) in player.iter_mut() {
        if collision.iter().any(|x| x.mover == entity) {
            return;
        }
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

        transform.translation.x += delta.0;
        transform.translation.y += delta.1;
        position.x += delta.0;
        position.y += delta.1;
    }
}

pub fn camera_follows_player(
    mut cameras: Query<&mut Transform, With<Camera>>,
    players: Query<&Position, With<Player>>
) {
    for player in players.iter() {
        for mut camera in cameras.iter_mut() {
            camera.translation.x = player.x;
            camera.translation.y = player.y;
        }
    }
}
