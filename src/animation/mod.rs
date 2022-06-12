use crate::prelude::*;
use bevy::asset::LoadState;

pub enum AnimDirection {
    Up,
    Down,
    Left,
    Right
}

#[derive(Default)]
pub struct RpgSpriteHandles {
    handles: Vec<HandleUntyped>
}

pub struct Animation;

impl Plugin for Animation {
    fn build(&self, app: &mut App) {
        app.init_resource::<RpgSpriteHandles>()
            .add_system_set(
                SystemSet::on_enter(Stages::Prepare)
                .with_system(load_textures)
                .with_system(check_textures)
            );
    }
}

fn load_textures(
    mut rpg_sprite_handles: ResMut<RpgSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    rpg_sprite_handles.handles = asset_server.load_folder("resources").unwrap();
}

fn check_textures(
    mut state: ResMut<State<Stages>>,
    rpg_sprite_handles: ResMut<RpgSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded = asset_server.get_group_load_state(rpg_sprite_handles.handles.iter().map(|handle| handle.id)) {
        state.set(Stages::MakeMap).unwrap();
    }
}
