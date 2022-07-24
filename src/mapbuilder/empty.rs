use crate::prelude::*;
use super::MapArchitect;

pub struct EmptyArchitect {}

impl MapArchitect for EmptyArchitect {
    fn new(&mut self, rng: &mut ThreadRng) -> MapBuilder {
        let mut mb = MapBuilder{
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Position::zero(),
            //amulet: Position::zero(),
            //theme: super::themes::DungeonTheme::new(),
        };

        mb.fill(TileType::Floor);
        mb.player_start = Position::new(SCREEN_WIDTH/2.0, SCREEN_HEIGHT/2.0);
        //mb.amulet = mb.find_most_distant();
        for _ in 0..1 {
            mb.monster_spawns.push(
                Position::new(
                    rng.gen_range(1..SCREEN_WIDTH as i32) as f32,
                    rng.gen_range(1..SCREEN_WIDTH as i32) as f32,
                )
            )
        }
        mb
    }
}
