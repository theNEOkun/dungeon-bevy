use crate::prelude::*;

impl Plugin for Player {
    fn build(&self, app: &mut App) {
        app.add_system(spawn).add_system(player_movement);
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    for mut transform in player.iter_mut() {
        let delta = if keyboard_input.pressed(KeyCode::Left) {
            Position::new(2.0, 0.0)
        } else if keyboard_input.pressed(KeyCode::Right) {
            Position::new(-2.0, 0.0)
        } else if keyboard_input.pressed(KeyCode::Up) {
            Position::new(0.0, -2.0)
        } else if keyboard_input.pressed(KeyCode::Down) {
            Position::new(0.0, 2.0)
        } else {
            Position::zero()
        };
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
    }
}

fn spawn(mut commands: Commands) {
    commands
        .spawn_bundle(
            SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.7, 0.7, 0.7),
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(MapLevel(0))
        .insert(Health {
            current: 10,
            max: 10,
        })
        .insert(FieldOfView::new(8))
        .insert(Damage(1));
}
