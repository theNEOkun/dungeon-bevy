mod map;
mod components;
mod player;
mod systems;
mod camera;
mod mapbuilder;

mod prelude {
    pub use bevy::prelude::*;
    pub use rand::prelude::*;

    pub const SCREEN_WIDTH: f32 = 80.0;
    pub const SCREEN_HEIGHT: f32 = 50.0;

    pub use crate::components::*;
    pub use crate::player::*;
    pub use crate::mapbuilder::*;
    pub use crate::map::*;
    pub use crate::systems::*;
    pub use crate::GameOptions;
}

use prelude::*;

#[derive(Debug)]
pub struct GameOptions {
    player_start: Option<Position>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(StageLabel)]
pub enum Stages {
    Prepare,
    Start,
    Cleanup
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<CollisionEvent>()
        .add_event::<SpawnPlayer>()
        .add_stage(Stages::Prepare, SystemStage::parallel())
        .add_stage(Stages::Start, SystemStage::parallel())
        .add_stage(Stages::Cleanup, SystemStage::parallel())
        .add_plugin(MapBuilder)
        .add_plugin(Systems)
        .add_plugin(PlayerPlugin)
        .run();
}
