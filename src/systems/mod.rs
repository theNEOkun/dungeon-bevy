mod collisions;
mod movement;

use crate::prelude::*;
use bevy::core::FixedTimestep;
pub use collisions::*;

pub struct Systems;

impl Plugin for Systems {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(Stages::Start)
                .with_run_criteria(FixedTimestep::steps_per_second(20.0))
                .with_system(movement::check_for_collisions)
                .with_system(movement::animation)
        );
    }
}
