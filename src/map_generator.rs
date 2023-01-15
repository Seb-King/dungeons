use bevy::{prelude::*, utils::HashMap};
use rand::{rngs::ThreadRng, Rng};

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
    rng: ThreadRng,
}

#[derive(Component, Clone, PartialEq, Copy)]
pub enum TileType {
    Void,
    Floor,
    Wall,
}

pub struct TileMap {
    tile_map: HashMap<IVec2, TileType>,
}

impl TileMap {
    pub fn new(map: HashMap<IVec2, TileType>) -> TileMap {
        TileMap { tile_map: map }
    }

    pub fn get(&self, pos: IVec2) -> TileType {
        let tile_option = self.tile_map.get(&pos);
        if let Some(tile) = tile_option {
            return *tile;
        }

        return TileType::Void;
    }

    pub fn set(&mut self, pos: IVec2, tile_type: TileType) {
        self.tile_map.insert(pos, tile_type);
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl MapGenerator {
    pub fn new() -> MapGenerator {
        MapGenerator {
            rng: rand::thread_rng(),
        }
    }

    pub fn generate_random(&mut self) -> DungeonMap {
        let starting_room = self.generate_room();
        let starting_room_pos = Vec2::new(
            self.rng.gen_range(4..16) as f32,
            self.rng.gen_range(4..16) as f32,
        );
        let direction = self.choose_direction();
        let corridor = self.generate_corridor(direction);
        let corridor_pos = starting_room_pos + self.place_corridor(starting_room, direction);

        let rooms = vec![(starting_room, starting_room_pos)];

        let corridors: Vec<(Corridor, Vec2)> = vec![(corridor, corridor_pos)];

        DungeonMap {
            rooms,
            corridors,
            map_size: (80, 45),
        }
    }

    fn generate_room(&mut self) -> Room {
        Room::new(self.rng.gen_range(4..16), self.rng.gen_range(4..16))
    }

    fn choose_direction(&mut self) -> Direction {
        let num = self.rng.gen_range(0..4);

        match num {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => Direction::Up,
        }
    }

    fn generate_corridor(&mut self, direction: Direction) -> Corridor {
        let first_hall_vec = match direction {
            Direction::Up => Vec2::Y,
            Direction::Down => Vec2::NEG_Y,
            Direction::Right => Vec2::X,
            Direction::Left => Vec2::NEG_X,
        };

        let second_hall_vec = match direction {
            Direction::Up => Vec2::X,
            Direction::Down => Vec2::X,
            Direction::Right => Vec2::Y,
            Direction::Left => Vec2::Y,
        };

        let length1 = self.rng.gen_range(2..10) as f32;

        let sign = if self.rng.gen_bool(0.5) { 1 } else { -1 };
        let turn_length = (self.rng.gen_range(2..10) * sign) as f32;

        let first = Vec2::new(first_hall_vec.x * length1, first_hall_vec.y * length1);

        let second = Vec2::new(
            second_hall_vec.x * turn_length,
            second_hall_vec.y * turn_length,
        );

        Corridor {
            shape: (first, second),
        }
    }

    fn place_corridor(&mut self, room: Room, direction: Direction) -> Vec2 {
        let width = room.width - 1;
        let height = room.height - 1;

        let (x, y) = match direction {
            Direction::Up => (self.rng.gen_range(1..width), (height - 1)),
            Direction::Down => (self.rng.gen_range(1..width), 0),
            Direction::Right => (width - 1, self.rng.gen_range(1..height)),
            Direction::Left => (0, self.rng.gen_range(1..height)),
        };

        Vec2::new(x as f32, y as f32)
    }
}

pub struct DungeonMap {
    pub rooms: Vec<(Room, Vec2)>,
    pub corridors: Vec<(Corridor, Vec2)>,
    pub map_size: (u32, u32),
}

pub struct Corridor {
    pub shape: (Vec2, Vec2),
}

impl DungeonMap {
    pub fn get_tile_map(&self) -> TileMap {
        let grid: HashMap<IVec2, TileType> = HashMap::new();

        let mut tile_map = TileMap::new(grid);

        for (room, pos) in &self.rooms {
            for y in 0..room.height {
                for x in 0..room.width {
                    let on_border = y == 0 || y == room.height - 1 || x == 0 || x == room.width - 1;
                    let pos = IVec2::new((x as f32 + pos.x) as i32, (y as f32 + pos.y) as i32);
                    if on_border {
                        tile_map.set(pos, TileType::Wall);
                    } else {
                        tile_map.set(pos, TileType::Floor);
                    }
                }
            }
        }

        for (corridor, starting_pos) in &self.corridors {
            let x_offset = starting_pos.x as i32;
            let y_offset = starting_pos.y as i32;

            let first_hall_length = corridor.shape.0.length() as u32;
            let second_hall_length = corridor.shape.1.length() as u32;

            let first_hall_dir = corridor.shape.0.normalize_or_zero();

            let mut s = IVec2::ZERO;

            for i in 0..first_hall_length {
                let x: i32 = x_offset + (i as i32) * (first_hall_dir.x as i32);
                let y: i32 = y_offset + (i as i32) * (first_hall_dir.y as i32);
                tile_map.set(IVec2::new(x, y), TileType::Floor);

                s = IVec2::new(x, y);
            }

            let second_hall_dir = corridor.shape.1.normalize_or_zero();
            for i in 0..second_hall_length {
                let x: i32 = s.x + ((i + 1) as i32) * (second_hall_dir.x as i32);
                let y: i32 = s.y + ((i + 1) as i32) * (second_hall_dir.y as i32);
                tile_map.set(IVec2::new(x, y), TileType::Floor);
            }
        }
        return tile_map;
    }
}
