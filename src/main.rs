mod map;
mod components;
mod player;
mod systems;
mod camera;
mod mapbuilder;

mod prelude {
    pub use bevy::prelude::*;
    pub const SCREEN_WIDTH: f32 = 80.0;
    pub const SCREEN_HEIGHT: f32 = 50.0;
    pub use crate::map::Position;
    pub use crate::components::*;
    pub use crate::player::*;
    pub use crate::mapbuilder::*;
}

use prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Player)
        .add_plugin(MapBuilder)
        .run();
}
