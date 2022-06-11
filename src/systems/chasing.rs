use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(FieldOfView)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(#[resource] map: &Map, ecs: &SubWorld, commands: &mut CommandBuffer) {
    let mut moving_enemies = <(Entity, &Point, &ChasingPlayer, &FieldOfView)>::query();
    let mut positions = <(Entity, &Point, &Health)>::query();
    let mut player = <(&Point, &Player)>::query();

    let player_pos = player.iter(ecs).nth(0).unwrap().0;
    let player_idx = map_idx(player_pos);

    moving_enemies.iter(ecs).for_each(|(entity, pos, _, fov)| {
        // If the player is not in fov, stop
        if !fov.visible_tiles.contains(&player_pos) {
            return;
        }

        let search_targets = vec![player_idx];

        // Create new djikstra-map searching for the player
        let djikstra_map =
            DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

        let idx = map_idx(pos);

        // Find the lowest position to move to
        if let Some(destination) = DijkstraMap::find_lowest_exit(&djikstra_map, idx, map) {
            // Get the pythagorean distance to the player
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);

            //Get the destination from that distance
            let destination = if distance > 1.2 {
                map.index_to_point2d(destination)
            } else {
                *player_pos
            };

            let mut attacked = false;

            // For each position with an entity
            positions
                .iter(ecs)
                .filter(|(_, target_pos, _)| **target_pos == destination) //If the position is what it wants to move to
                .for_each(|(victim, _pos, _)| {
                    // If the player is in the square, then move to attack
                    if ecs
                        .entry_ref(*victim)
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok()
                    {
                        commands.push((
                            (),
                            WantsToAttack {
                                attacker: *entity,
                                victim: *victim,
                            },
                        ));
                    }
                    attacked = true;
                });

            // If the char has not attacked, then move into that square
            if !attacked {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            }
        }
    })
}
