mod movement;

use crate::prelude::*;
use bevy::core::FixedTimestep;

pub struct Systems;

impl Plugin for Systems {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(Stages::Start)
                .with_system(movement::check_for_collisions)
                .with_system(movement::animation.before(movement::check_for_collisions))
        );
    }
}
