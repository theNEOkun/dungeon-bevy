use crate::prelude::*;
use bevy::asset::LoadState;

#[derive(Clone, Copy, Debug)]
pub enum AnimDirection {
    Up = 16*2,
    Down = 16*0,
    Left = 16*1,
    Right = 16*3
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
    rpg_sprite_handles.handles = asset_server.load_folder("textures/").unwrap();
}

fn check_textures(
    mut state: ResMut<State<Stages>>,
    rpg_sprite_handles: ResMut<RpgSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    if let LoadState::Loaded = asset_server.get_group_load_state(rpg_sprite_handles.handles.iter().map(|handle| handle.id)) {
        let mut texture_atlas_builder = TextureAtlasBuilder::default();

        for handle in &rpg_sprite_handles.handles {
            let new_handle = handle.clone_weak().typed();
            let texture = textures.get(&new_handle).expect("Textures folder contained a file which did not create an Image");
            texture_atlas_builder.add_texture(new_handle, texture);
        }

        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        texture_atlases.add(texture_atlas);

        state.set(Stages::MakeMap).unwrap();
    }
}
