mod rect;
mod empty;
mod rooms;

use crate::prelude::*;
use bevy::asset::LoadState;
use rect::Rect;

const NUM_ROOMS: usize = 20;

trait MapArchitect {
    fn new(&mut self, rng: &mut ThreadRng) -> MapBuilder;
}

use empty::EmptyArchitect;
use rooms::RoomsArchitect;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Position,
}

impl MapBuilder {
    pub fn new(rng: &mut ThreadRng) -> Self {
        let mut arch = EmptyArchitect{};
        arch.new(rng)
    }

    /// Fills the map with a certain tile
    /// Mostly used with the wall-tile
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    /// Randomly builds the rooms, up to 20 per map
    fn build_random_rooms(&mut self, rng: &mut ThreadRng) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.gen_range(1..=SCREEN_WIDTH as i32 - 10),
                rng.gen_range(1..=SCREEN_HEIGHT as i32 - 10),
                rng.gen_range(2..=10),
                rng.gen_range(2..=10),
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH as i32 && p.y > 0 && p.y < SCREEN_HEIGHT as i32
                    {
                        self.map[&p] = TileType::Floor;
                    }
                });

                self.rooms.push(room);
            }
        }
    }

    /// Builds the random corridors
    fn build_corridors(&mut self, rng: &mut ThreadRng) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));
        for (index, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[index - 1].center();
            let new = room.center();

            if rng.gen_range(0..2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }

    /// Creates vertical tunnels
    /// From y1 to y2, on the x-axis
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};

        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(PositionI::new(x, y)) {
                self.map[&idx] = TileType::Floor;
            }
        }
    }

    /// Creates horizontal tunnels for the map
    /// From x1 to x2, on the y-axis
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};

        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(PositionI::new(x, y)) {
                self.map[&idx] = TileType::Floor;
            }
        }
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapBuilder::new(&mut thread_rng()));
        app.add_startup_system(make_map);
    }
}

pub fn make_map(
    mut commands: Commands,
    mut options: ResMut<GameOptions>,
    mut state: ResMut<State<Stages>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mb: Res<MapBuilder>,
) {
    let texture = asset_server.load("textures/dungeonfont.png");
    let texture_atlas = TextureAtlas::from_grid(texture, Vec2::new(32.0, 32.0), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    for y in 0..=(SCREEN_HEIGHT - 1.0) as usize {
        for x in 0..=(SCREEN_WIDTH - 1.0) as usize {
            let pos = Position::new_from_usize(x, y);
            let tile = mb.map[&pos];
            let (tile, extra) = match tile {
                TileType::Wall => (b'#' as usize, Some(Wall)),
                TileType::Floor => (b'.' as usize, None),
                _ => (0, None),
            };
            let mut sprite = commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: tile,
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(x as f32, y as f32, 1.0),
                    ..default()
                },
                ..default()
            });
            sprite.insert(pos);
            if let Some(extra) = extra {
                sprite.insert(extra);
            }
        }
    }
    options.player_start = mb.player_start;
    state.set(Stages::Start).unwrap();
}
