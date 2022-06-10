mod map;
mod components;
mod player;

mod prelude {
    pub use bevy::prelude::*;
    pub const SCREEN_WIDTH: f32 = 80.0;
    pub const SCREEN_HEIGHT: f32 = 50.0;
    pub use crate::map::Position;
    pub use crate::components::*;
    pub use crate::player::*;
}

use prelude::*;

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Player)
        .add_startup_system(setup_camera)
        .run();
}
