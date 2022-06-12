mod map;
mod components;
mod player;
mod systems;
mod camera;
mod mapbuilder;
mod animation;

mod prelude {
    pub use bevy::prelude::*;
    pub use rand::prelude::*;

    pub const SCREEN_WIDTH: f32 = 80.0;
    pub const SCREEN_HEIGHT: f32 = 50.0;

    pub const TIME_STEP: f32 = 2.0 / 60.0;

    pub use crate::components::*;
    pub use crate::player::*;
    pub use crate::mapbuilder::*;
    pub use crate::map::*;
    pub use crate::systems::*;
    pub use crate::camera::*;
    pub use crate::animation::*;
    pub use crate::*;
}

use prelude::*;

#[derive(Debug)]
pub struct GameOptions {
    player_start: Position,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(StageLabel)]
pub enum Stages {
    Prepare,
    MakeMap,
    Start,
    Cleanup
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<CheckCollision>()
        .add_event::<WantsToMove>()
        .insert_resource(GameOptions{ player_start: Position::zero() })
        .insert_resource(
            WindowDescriptor {
                title: "DungeonCrawler".to_string(),
                width: 1024.0,
                height: 768.0,
                ..default()
            }
        )
        .add_state(Stages::Prepare)
        .add_plugin(Animation)
        .add_plugin(MapBuilder)
        .add_plugin(Systems)
        .add_plugin(PlayerPlugin)
        .run();
}
