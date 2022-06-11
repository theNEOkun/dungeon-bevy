use crate::prelude::*;
use bevy::log::*;

#[derive(Clone, Copy)]
pub struct Rect {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rect {
    pub fn with_size(x1: i32, y1: i32, width: i32, height: i32) -> Self {
        Self {
            x1,
            y1,
            x2: x1 + width,
            y2: y1 + height,
        }
    }

    /// Used to find the center of a Rect
    pub fn center(&self) -> PositionI {
        PositionI::new((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    /// Used to check if another rect intersects with this one
    pub fn intersect(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(PositionI),
    {
        for y in self.y1..=self.y2 {
            for x in self.x1..=self.x2 {
                f(PositionI::new(x, y))
            }
        }
    }
}

impl std::fmt::Debug for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(x1: {}, y1: {}), (x1: {}, y2: {})",
            self.x1, self.y1, self.x2, self.y2
        )
    }
}

const NUM_ROOMS: usize = 20;

pub struct MapArch {
    pub map: Map,
    pub rooms: Vec<Rect>,
}

impl MapArch {
    pub fn new(rng: &mut ThreadRng) -> Self {
        let mut arch = MapArch {
            map: Map::new(),
            rooms: Vec::new(),
        };

        arch.fill(TileType::Wall);
        arch.build_random_rooms(rng);
        arch.build_corridors(rng);

        arch
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

fn make_map(mut commands: Commands) {
    let map_builder = MapArch::new(&mut thread_rng());
    for y in 0..=(SCREEN_HEIGHT - 1.0) as usize {
        for x in 0..=(SCREEN_WIDTH - 1.0) as usize {
            let pos = Position::new_from_usize(x, y);
            match map_builder.map[&pos] {
                TileType::Wall => commands.spawn_bundle(
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::YELLOW,
                            custom_size: Some(Vec2::new(1.0, 1.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x as f32, y as f32, 0.1),
                        ..default()
                    })
                    .insert(pos)
                    .insert(Wall),
                TileType::Floor => commands.spawn_bundle(
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::BLUE,
                            custom_size: Some(Vec2::new(1.0, 1.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x as f32, y as f32, 0.1),
                        ..default()
                    })
                    .insert(pos),
                _ => commands.spawn_bundle(
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::RED,
                            custom_size: Some(Vec2::new(1.0, 1.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x as f32, y as f32, 0.1),
                        ..default()
                    })
                    .insert(pos),
            };
        }
    }
    commands.insert_resource(map_builder.map);
}

pub struct MapBuilder;

impl Plugin for MapBuilder {
    fn build(&self, app: &mut App) {
        app.add_startup_system(make_map);
    }
}
