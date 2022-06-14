use crate::prelude::*;

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
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(16.0, 32.0), 16, 8);
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
        .insert(player_start)
        .insert(Player)
        .insert(AnimDirection::Down)
        .insert(Animations {
            walking: Animated {
                timer: Timer::from_seconds(0.1, true),
                offset: 0,
                length: 4,
                counter: 0,
            },
            attacking: Animated {
                timer: Timer::from_seconds(0.5, true),
                offset: 5,
                length: 3,
                counter: 0,
            },
        })
        .insert(Collider);
    commands.spawn_bundle(new_camera_2d());
}

pub fn player_attacking(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<
        (
            Entity,
            &mut Transform,
            &Visibility,
        ),
        (With<Player>, Without<AttackAnim>),
    >,
    mut event_writer: EventWriter<WantsToAttack>,
) {
    for (entity, _, visible) in player.iter_mut() {
        if !visible.is_visible {
            return;
        }
        if keyboard_input.just_pressed(KeyCode::Space) {
            event_writer.send(WantsToAttack {
                attacker: entity,
            });
            commands.entity(entity).insert(AttackAnim);
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
        let anim_dir = if destination.x != 0.0 {
            if destination.x < 0.0 {
                AnimDirection::Right
            } else {
                AnimDirection::Left
            }
        } else if destination.y != 0.0 {
            if destination.y < 0.0 {
                AnimDirection::Down
            } else {
                AnimDirection::Up
            }
        } else {
            *direction
        };
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
