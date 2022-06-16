mod map;
mod components;
mod player;
mod systems;
mod camera;
mod mapbuilder;

mod prelude {
    pub use bevy::prelude::*;
    pub use rand::prelude::*;
    pub use bevy_rapier2d::prelude::*;

    pub const SCREEN_WIDTH: f32 = 80.0;
    pub const SCREEN_HEIGHT: f32 = 50.0;

    pub const TIME_STEP: f64 = 1.0 / 10.0;

    pub use crate::components::*;
    pub use crate::player::*;
    pub use crate::mapbuilder::*;
    pub use crate::map::*;
    pub use crate::systems::*;
    pub use crate::camera::*;
    pub use crate::*;
}

use prelude::*;

#[derive(Component, Copy, Clone, PartialEq, Debug)]
pub enum AnimDirection {
    Up = 16 * 2,
    Down = 16 * 0,
    Left = 16 * 1,
    Right = 16 * 3,
}

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
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0)) 
        .add_event::<CheckCollision>()
        .add_event::<WantsToMove>()
        .add_event::<WantsToAttack>()
        .add_event::<Animate>()
        .insert_resource(GameOptions{ player_start: Position::zero() })
        .insert_resource(
            WindowDescriptor {
                title: "DungeonCrawler".to_string(),
                width: 1024.0,
                height: 768.0,
                ..default()
            }
        )
        .add_state(Stages::MakeMap)
        .add_plugin(MapPlugin)
        .add_plugin(Systems)
        .add_plugin(PlayerPlugin)
        .run();
}
