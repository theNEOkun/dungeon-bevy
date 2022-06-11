mod collisions;
mod movement;

use crate::prelude::*;
pub use collisions::*;

pub struct Systems;

impl Plugin for Systems {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(scheduler());
    }
}

pub fn scheduler() -> SystemSet {
    SystemSet::new()
        .with_system(collisions::check_for_collisions)
        .with_system(movement::movement)
}
