use super::camera::new_camera_2d;
use crate::prelude::*;

impl Plugin for Player {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .add_system(player_movement)
            .add_system(camera_follows_player);
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Position), With<Player>>,
) {
    for (mut transform, mut position) in player.iter_mut() {
        let mut delta = (0.0, 0.0);
        if keyboard_input.pressed(KeyCode::Left) {
            delta.0 = 2.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            delta.0 = -2.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            delta.1 = -2.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            delta.1 = 2.0;
        }

        transform.translation.x = delta.0;
        transform.translation.y = delta.1;
        position.x = delta.0;
        position.y = delta.1;
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

fn spawn_player(mut commands: Commands) {
    commands
        .spawn_bundle(
            SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.7),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(3.0, 3.0, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(Position::zero())
        .insert(Player)
        .insert(MapLevel(0))
        .insert(Health {
            current: 10,
            max: 10,
        })
        .insert(FieldOfView::new(8))
        .insert(Damage(1));
    commands.spawn_bundle(new_camera_2d());
}
