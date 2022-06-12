mod collisions;
mod movement;

use crate::prelude::*;
use bevy::core::FixedTimestep;
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
        .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
        //.with_system(collisions::check_for_collisions)
        //.with_system(movement::movement)
}
