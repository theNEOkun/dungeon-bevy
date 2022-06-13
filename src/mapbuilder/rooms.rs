use crate::prelude::*;
use super::MapArchitect;

pub struct RoomsArchitect{}

impl MapArchitect for RoomsArchitect {
    fn new(&mut self, rng: &mut ThreadRng) -> MapBuilder {
        let mut mb = MapBuilder{
            map: Map::new(),
            rooms: Vec::new(),
            //monster_spawns: Vec::new(),
            player_start: Position::zero(),
            //amulet: Point::zero(),
            //theme: super::themes::DungeonTheme::new(),
        };

        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = Position::new_from_position(mb.rooms[0].center());
        //mb.amulet = mb.find_most_distant();
        //for room in mb.rooms.iter().skip(1) {
        //    mb.monster_spawns.push(room.center());
        //}
        mb
    }
}