use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        // Moves the character from point a to point b
        commands.add_component(want_move.entity, want_move.destination);

        // if there is a move to be made
        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            // if there is fov
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                // Update fov
                commands.add_component(want_move.entity, fov.clone_dirty());

                // Change the position of the camera
                if entry.get_component::<Player>().is_ok() {
                    camera.on_player_move(want_move.destination);
                    fov.visible_tiles.iter().for_each(|pos| {
                        map.revealed_tiles[map_idx(pos)] = true;
                    });
                }
            }
        }
        commands.remove(*entity);
    }
}
