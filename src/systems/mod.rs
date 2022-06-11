use crate::prelude::*;

pub struct Systems;

impl Plugin for Systems {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(build_input_scheduler());
    }
}

pub fn build_input_scheduler() -> SystemSet {
    SystemSet::new()
}

pub fn build_player_scheduler() -> SystemSet {
    SystemSet::new()
}

pub fn build_monster_scheduler() -> SystemSet {
    SystemSet::new()
}
