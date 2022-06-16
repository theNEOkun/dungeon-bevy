mod movement;
mod combat;

use crate::prelude::*;
use bevy::core::FixedTimestep;

pub struct Systems;

impl Plugin for Systems {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(Stages::Start)
                //.with_system(movement::check_for_collisions)
                .with_system(movement::walking_animation)
        );
        app.add_system_set(
            SystemSet::on_update(Stages::Start)
            .with_system(combat::attack_animation)
            .with_system(combat::attack)
        );
    }
}
