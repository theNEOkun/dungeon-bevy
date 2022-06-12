use bevy::{core::FixedTimestep, render::render_resource::Texture};

use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(Stages::Start)
                .with_system(spawn_player)
        );
        app.add_system_set(
            SystemSet::on_update(Stages::Start)
                .with_system(player_movement)
                .with_system(camera_follows_player.after(check_collision)),
        );
        app.add_system_set(
            SystemSet::on_update(Stages::Start)
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(check_collision.after(player_movement))
        );
    }
}

pub fn spawn_player(
    mut commands: Commands,
    options: Res<GameOptions>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
    let player_start = options.player_start;
    let texture = asset_server.load("textures/character.png");
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(16.0, 32.0), 16, 8);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(
            SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(1),
            transform: Transform {
                translation: Vec3::new(player_start.x, player_start.y, 100.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(player_start)
        .insert(Player)
        .insert(Animated {
            frame: 0,
            direction: AnimDirection::Down,
        })
        .insert(Collider);
    commands.spawn_bundle(new_camera_2d());
}

pub fn check_collision(
    mut player: Query<(Entity, &mut Position, &mut Transform), With<Collider>>,
    walls: Query<(Entity, &Position, &Transform), (With<Wall>, Without<Collider>)>,
    mut event_reader: EventReader<WantsToMove>,
    map: Res<Map>,
) {
    for (_, mut position, mut transform) in player.iter_mut() {
        for (_, _, _) in walls.iter() {
            for each in event_reader.iter() {
                let can_move = map.can_enter_tile_f(&each.destination);
                if can_move {
                    position.x = each.destination.x;
                    position.y = each.destination.y;
                    transform.translation.x = position.x;
                    transform.translation.y = position.y;
                } else {
                    println!("{position:?}");
                }
            }
        }
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<(Entity, &mut Transform, &mut Position), With<Player>>,
    mut event_writer: EventWriter<WantsToMove>,
) {
    for (entity, _, position) in player.iter_mut() {
        let mut delta = (0.0, 0.0);
        if keyboard_input.pressed(KeyCode::Left) {
            delta.0 = -1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            delta.0 = 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            delta.1 = 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            delta.1 = -1.0;
        }
        let destination = Position::new(position.x + delta.0, position.y + delta.1);
        if delta != (0.0, 0.0) {
            event_writer.send(WantsToMove {
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
