mod movement;
mod combat;

use crate::prelude::*;
use bevy::core::FixedTimestep;

pub struct Systems;

impl Plugin for Systems {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(Stages::Start)
                .with_run_criteria(FixedTimestep::step(TIME_STEP * 10.0))
                .with_system(movement::chasing)
        );
        app.add_system_set(
            SystemSet::on_update(Stages::Start)
                .with_system(movement::make_move)
                .with_system(movement::check_for_collisions)
                .with_system(movement::walking_animation)
        );
        app.add_system_set(
            SystemSet::on_update(Stages::Start)
            .with_system(combat::attack_animation)
            .with_system(combat::on_attack)
            .with_system(combat::attack)
            .with_system(combat::after_attack)
        );
    }
}
