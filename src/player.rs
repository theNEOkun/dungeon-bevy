use crate::prelude::*;
use bevy::sprite::Rect;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(Stages::Start).with_system(spawn_player));
        app.add_system_set(
            SystemSet::on_update(Stages::Start)
                .with_system(player_movement)
                .with_system(player_attacking)
                .with_system(camera_follows_player.after(player_movement)),
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
    let mut texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(16.0, 32.0), 16, 4);
    add_attack_anims(&mut texture_atlas, 4.0 * 32.0, 32);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                index: 0,
                custom_size: Some(Vec2::new(1.0, 2.0)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(player_start.x, player_start.y, 100.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            visibility: Visibility { is_visible: true },
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Collider::capsule_y(0.01, 0.45))
                .insert(Transform::from_xyz(0.0, 1000.0, 0.0));
        })
        .insert(GravityScale(0.0))
        .insert(Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        })
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Sleeping::disabled())
        .insert(Player)
        .insert(AnimDirection::Down)
        .insert(Living {
            speed: 3.0,
            current_hp: 8,
            max_hp: 8,
        })
        .insert(Weapon {
            damage: 1,
            damage_frames: vec![1, 2],
        })
        .insert(Animations {
            walking: Animated {
                timer: Timer::from_seconds(0.2, true),
                offset: 0,
                length: 4,
                counter: 0,
            },
            attacking: Animated {
                timer: Timer::from_seconds(0.2, true),
                offset: (16 * 4),
                length: 4,
                counter: 0,
            },
        });
    commands.spawn_bundle(new_camera_2d());
}

fn add_attack_anims(atlas: &mut TextureAtlas, curr_y: f32, size: usize) {
    let curr_y = curr_y as i32;
    let mut iteration = 0;
    'outer: for y in (curr_y..(curr_y * 4 * size as i32)).step_by(size) {
        for x in (0..(4 * size)).step_by(size) {
            let min = Vec2::new(x as f32, y as f32);
            let max = Vec2::new((x + size) as f32, (y + size as i32) as f32);
            atlas.add_texture(Rect { min, max });
        }
        iteration += 1;
        if iteration >= 4 {
            break 'outer;
        }
    }
}

pub fn player_attacking(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<(Entity, &Visibility), (With<Player>, Without<AttackAnim>)>,
    mut event_writer: EventWriter<WantsToAttack>,
) {
    for (entity, visible) in player.iter_mut() {
        if !visible.is_visible {
            return;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            event_writer.send(WantsToAttack { attacker: entity });
        }
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<
        (
            Entity,
            &mut Transform,
            &mut AnimDirection,
            &mut TextureAtlasSprite,
            &Visibility,
        ),
        (With<Player>, Without<AttackAnim>),
    >,
    mut event_writer: EventWriter<WantsToMove>,
) {
    for (entity, _, mut direction, mut sprite, visible) in player.iter_mut() {
        if !visible.is_visible {
            return;
        }
        let destination = if keyboard_input.pressed(KeyCode::A) {
            Position::new(-1.0, 0.0)
        } else if keyboard_input.pressed(KeyCode::D) {
            Position::new(1.0, 0.0)
        } else if keyboard_input.pressed(KeyCode::W) {
            Position::new(0.0, 1.0)
        } else if keyboard_input.pressed(KeyCode::S) {
            Position::new(0.0, -1.0)
        } else {
            Position::zero()
        };
        let anim_dir = AnimDirection::match_position_prev(destination, *direction);
        if !destination.is_zero() {
            let destination = destination.normalize();
            event_writer.send(WantsToMove {
                entity,
                destination,
            });
        } else {
            sprite.index = anim_dir as usize;
        }
        if anim_dir != *direction {
            *direction = anim_dir;
        }
    }
}

/// Handles moving the camera along with the player
pub fn camera_follows_player(
    mut cameras: Query<&mut Transform, With<Camera>>,
    players: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    for player in players.iter() {
        for mut camera in cameras.iter_mut() {
            camera.translation.x = player.translation.x;
            camera.translation.y = player.translation.y;
        }
    }
}
