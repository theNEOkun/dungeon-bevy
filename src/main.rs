mod map;
mod components;

mod prelude {
    pub use bevy::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::Position;
}

use prelude::*;

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .run();
}
