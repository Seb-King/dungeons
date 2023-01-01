use bevy::prelude::*;
use rand::Rng;

#[derive(Clone, Copy)]
pub struct Room {
    width: u32,
    height: u32,
}

impl Room {
    fn new(width: u32, height: u32) -> Room {
        Room { width, height }
    }
}

pub struct MapGenerator {
    pub map: Vec<(Room, Vec2)>,
    pub map_size: (u32, u32)
}

#[derive(Component, Clone, PartialEq, Copy)]
pub enum TileType {
    Void,
    Floor,
    Wall,
}

pub struct TileMap {
    tile_map: Vec<Vec<TileType>>
}

impl TileMap {
    fn new(map: Vec<Vec<TileType>>) -> TileMap {
        TileMap { tile_map: map }
    }
}

pub trait Access {
    fn get(&self, x: u32, y: u32) -> TileType;
    fn set(&mut self, x: u32, y: u32, val: TileType) -> Option<TileType>;
}

impl Access for TileMap {
    fn get(&self, x: u32, y: u32) -> TileType {
        let row_option = self.tile_map.get(y as usize);
        if let Some(row) = row_option {
            let tile_option = row.get(x as usize);
            if let Some(tile_type) = tile_option {
                return *tile_type;
            }
        }

        return TileType::Void;
    }

    fn set(&mut self, x: u32, y: u32, val: TileType) -> Option<TileType> {
        self.tile_map[y as usize][x as usize] = val;

        return Option::Some(val);
    }
}

pub trait Grid {
    fn get_tile_map(&self) -> TileMap;
}

impl MapGenerator {
    pub fn generate() -> DungeonMap {
        let map = vec![(Room::new(12, 30), Vec2::ZERO), (Room::new(12, 30), Vec2::new(15.0, 0.0))];

        DungeonMap { map, map_size: (80, 45) }
    }

    pub fn generate_random() -> DungeonMap {
        let mut rng = rand::thread_rng();

        let map = vec![(Room::new(rng.gen_range(4..16), rng.gen_range(4..16)), Vec2::ZERO), (Room::new(rng.gen_range(4..16), rng.gen_range(4..16)), Vec2::new(15.0, 0.0))];

        DungeonMap { map, map_size: (80, 45) }
    }
}

pub struct DungeonMap {
    pub map: Vec<(Room, Vec2)>,
    pub map_size: (u32, u32)
}

impl Grid for DungeonMap {
    fn get_tile_map(&self) -> TileMap {
        let mut grid: Vec<Vec<TileType>> = Vec::new();

        let mut row: Vec<TileType> = Vec::new();

        for _ in 0..self.map_size.0 {
            row.push(TileType::Void);
        }

        for _ in 0..self.map_size.1 {
            grid.push(row.clone())            
        }

        let mut tile_map = TileMap::new(grid);

        for (room, pos) in &self.map {
            for y in 0..room.height {                
                for x in 0..room.width {

                    let on_border = y == 0 || y == room.height - 1 || x == 0 || x == room.width - 1;

                    if on_border {
                        tile_map.set(x + pos.x as u32, y + pos.y as u32, TileType::Wall);
                    } else {
                        tile_map.set(x + pos.x as u32, y + pos.y as u32, TileType::Floor);
                    }
                }
            }
        }
        return tile_map;
    }
}
