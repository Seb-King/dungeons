use crate::dungeon_generation::dungeon_state::{DungeonLayout, DungeonState, DungeonStateBuilder};
use crate::dungeon_generation::room::Orientation::{DOWN, LEFT, RIGHT, UP};
use crate::dungeon_generation::room::{Collision, Corridor, IShape, Rectangle, Room};
use bevy::prelude::IVec2;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct DungeonGenerator {
    steps: Vec<BoxedStep<DungeonState>>,
}

type Step<T> = fn(&T) -> Result<T, String>;
type BoxedStep<T> = Box<dyn Fn(&T) -> Result<T, String>>;

impl DungeonGenerator {
    #[allow(dead_code)]
    pub fn new() -> DungeonGenerator {
        DungeonGenerator { steps: Vec::new() }
    }

    pub fn generate(&self) -> Result<DungeonState, String> {
        let layout = DungeonLayout::new();

        let state = DungeonState {
            layout,
            spawns: Vec::new(),
            rng: Rc::new(RefCell::new(rand::thread_rng())),
        };

        let new_state: Result<DungeonState, String> = self
            .steps
            .iter()
            .try_fold(state, |result, step| step(&result));

        return new_state;
    }

    pub fn add_step(mut self, step: Step<DungeonState>) -> Self {
        self.steps.push(Box::new(step));
        self
    }

    pub fn add_retryable_step(mut self, step: Step<DungeonState>) -> Self {
        let max_retries = 1000;
        let retry_step_until_ok = move |s: &DungeonState| -> Result<DungeonState, String> {
            let mut retry_count = 0;
            let mut attempt: Result<DungeonState, String> = Err("First attempt".to_string());
            while attempt.is_err() && retry_count < max_retries {
                attempt = step(s);
                retry_count += 1;
            }
            attempt.map_err(|err| err + " and exceeded maximum retries")
        };
        self.steps.push(Box::new(retry_step_until_ok));
        self
    }
}

impl DungeonLayout {
    fn new() -> Self {
        DungeonLayout {
            rooms: Vec::new(),
            corridors: Vec::new(),
        }
    }
}

pub fn add_room(state: &DungeonState) -> Result<DungeonState, String> {
    let mut rng = state.rng.borrow_mut();

    let width = rng.gen_range(6..16);
    let height = rng.gen_range(6..16);

    let position = if let Some(corridor) = state.layout.corridors.last() {
        let joining_corridor_pos = corridor.position
            + IVec2::from(corridor.shape.orientation) * (corridor.shape.length as i32 - 1);

        let (x, y) = match corridor.shape.orientation {
            DOWN => (rng.gen_range(1..(width - 1)), height - 1),
            UP => (rng.gen_range(1..(width - 1)), 0),
            LEFT => (width - 1, rng.gen_range(1..(height - 1))),
            RIGHT => (0, rng.gen_range(1..(height - 1))),
        };

        joining_corridor_pos - IVec2::new(x as i32, y as i32)
    } else {
        IVec2::new(rng.gen_range(20..25), rng.gen_range(20..25))
    };

    let room = Room {
        shape: Rectangle { width, height },
        position,
    };

    let mut rooms = state.layout.rooms.clone();

    let disjoint_rooms = rooms.iter().all(|r| !r.collides_with(&room));
    let disjoint_from_corridors = state
        .layout
        .corridors
        .iter()
        .all(|c| !c.collides_with(&room));

    if disjoint_rooms && disjoint_from_corridors {
        rooms.push(room);

        return Ok(DungeonStateBuilder::from_state(state).rooms(rooms).build());
    }

    Err("Failed to add room".to_string())
}

pub fn add_corridor(state: &DungeonState) -> Result<DungeonState, String> {
    let mut rng = state.rng.borrow_mut();

    let num = rng.gen_range(0..4);

    let orientation = match num {
        0 => UP,
        1 => DOWN,
        2 => LEFT,
        3 => RIGHT,
        _ => UP,
    };

    let index = rng.gen_range(0..state.layout.rooms.len());
    let random_room = state.layout.rooms.get(index);

    let position = if let Some(room) = random_room {
        let width = room.shape.width - 1;
        let height = room.shape.height - 1;

        let offset = match orientation {
            UP => IVec2::new(rng.gen_range(1..width) as i32, height as i32),
            DOWN => IVec2::new(rng.gen_range(1..width) as i32, 0),
            RIGHT => IVec2::new(width as i32, rng.gen_range(1..height) as i32),
            LEFT => IVec2::new(0, rng.gen_range(1..height) as i32),
        };

        offset + room.position
    } else {
        IVec2::new(0, 0)
    };

    let corridor = Corridor {
        shape: IShape {
            orientation,
            length: rng.gen_range(3..12),
        },
        position,
    };

    let mut corridors = state.layout.corridors.clone();

    let disjoint_from_corridors = corridors.iter().all(|c| !c.collides_with(&corridor));
    let disjoint_from_rooms = state
        .layout
        .rooms
        .iter()
        .all(|r| !r.collides_with(&corridor));

    if disjoint_from_corridors && disjoint_from_rooms {
        corridors.push(corridor);

        return Ok(DungeonStateBuilder::from_state(state)
            .corridors(corridors)
            .build());
    }

    Err("Failed to add corridor".to_string())
}

pub fn add_corridor_then_room(state: &DungeonState) -> Result<DungeonState, String> {
    return add_corridor(state).and_then(|res| add_room(&res));
}

#[cfg(test)]
mod dungeon_builder_tests {
    use super::*;

    #[test]
    fn add_room_works() {
        let builder = DungeonGenerator::new().add_step(add_room);

        assert_eq!(builder.generate().unwrap().layout.rooms.len(), 1);
    }

    #[test]
    fn add_two_rooms() {
        let builder = DungeonGenerator::new()
            .add_step(add_room)
            .add_step(add_room);

        assert_eq!(builder.generate().unwrap().layout.rooms.len(), 2);
    }

    #[test]
    fn add_multiple_rooms() {
        let builder = DungeonGenerator::new()
            .add_step(add_room)
            .add_step(add_room)
            .add_step(add_room);

        assert_eq!(builder.generate().unwrap().layout.rooms.len(), 3);
    }

    #[test]
    fn add_retryable_step() {
        let builder = DungeonGenerator::new()
            .add_retryable_step(add_room)
            .add_retryable_step(add_room);

        assert_eq!(builder.generate().unwrap().layout.rooms.len(), 2);
    }

    #[test]
    fn add_retryable_step_5_times() {
        let builder = DungeonGenerator::new()
            .add_retryable_step(add_room)
            .add_retryable_step(add_room)
            .add_retryable_step(add_room)
            .add_retryable_step(add_room)
            .add_retryable_step(add_room);

        let dungeon = builder.generate().unwrap().layout;

        assert_eq!(dungeon.rooms.len(), 5);
    }

    #[test]
    fn add_corridor_works() {
        let builder = DungeonGenerator::new().add_step(add_corridor);

        let layout = builder.generate().unwrap().layout;

        assert_eq!(layout.rooms.len(), 0);
        assert_eq!(layout.corridors.len(), 1);
    }

    #[test]
    fn add_room_then_corridor() {
        let builder = DungeonGenerator::new()
            .add_step(add_room)
            .add_step(add_corridor);

        let result = builder.generate();

        assert_eq!(result.is_ok(), true);
        let layout = result.unwrap().layout;
        assert_eq!(layout.rooms.len(), 1);
        assert_eq!(layout.corridors.len(), 1);
    }
}
