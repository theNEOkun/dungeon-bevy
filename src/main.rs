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
}

use prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MapBuilder)
        .add_plugin(Player)
        .run();
}
