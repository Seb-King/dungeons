use crate::dungeon_generation::room::Orientation::{DOWN, LEFT, RIGHT, UP};
use crate::dungeon_generation::room::{Collision, Corridor, IShape, Orientation, Rectangle, Room};
use bevy::prelude::IVec2;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

pub struct DungeonState {
    layout: DungeonLayout,
    rng: Rc<RefCell<ThreadRng>>,
}

#[derive(Clone, Debug)]
pub struct DungeonLayout {
    pub rooms: Vec<Room>,
    pub corridors: Vec<Corridor>,
}

pub struct DungeonGenerator {
    steps: Vec<BoxedStep<DungeonState>>,
}

trait Generator<T, Params> {
    fn new(params: Params) -> Self;

    fn generate(&self) -> T;
}

type Step<T> = fn(&T) -> Result<T, String>;
type BoxedStep<T> = Box<dyn Fn(&T) -> Result<T, String>>;

impl DungeonGenerator {
    pub fn new() -> DungeonGenerator {
        DungeonGenerator { steps: Vec::new() }
    }

    pub fn generate(&self) -> Result<DungeonLayout, String> {
        let layout = DungeonLayout::new();

        let state = DungeonState {
            layout,
            rng: Rc::new(RefCell::new(rand::thread_rng())),
        };

        let new_state: Result<DungeonState, String> = self
            .steps
            .iter()
            .try_fold(state, |result, step| step(&result));

        return new_state.map(|final_state| final_state.layout);
    }

    pub fn add_step(mut self, step: Step<DungeonState>) -> Self {
        self.steps.push(Box::new(step));
        self
    }

    pub fn add_retryable_step(mut self, step: Step<DungeonState>) -> Self {
        let retry_step_until_ok = move |s: &DungeonState| -> Result<DungeonState, String> {
            let mut attempt: Result<DungeonState, String> = Err("First attempt".to_string());
            while attempt.is_err() {
                attempt = step(s);
            }
            attempt
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

    let room = Room {
        shape: Rectangle {
            width: rng.gen_range(6..16),
            height: rng.gen_range(6..16),
        },
        position: IVec2::new(rng.gen_range(0..60), rng.gen_range(0..30)),
    };

    let mut rooms = state.layout.rooms.clone();

    let disjoint_rooms = rooms.iter().all(|r| !r.collides_with(&room));

    if disjoint_rooms {
        rooms.push(room);

        return Ok(DungeonState {
            layout: DungeonLayout {
                rooms,
                corridors: state.layout.corridors.clone(),
            },
            rng: Rc::clone(&state.rng),
        });
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

    let last_room = state.layout.rooms.last();
    let position = if let Some(room) = last_room {
        let width = room.shape.width - 1;
        let height = room.shape.height - 1;

        match orientation {
            UP => IVec2::new(rng.gen_range(1..width) as i32, height as i32),
            DOWN => IVec2::new(rng.gen_range(1..width) as i32, 0),
            RIGHT => IVec2::new(width as i32, rng.gen_range(1..height) as i32),
            LEFT => IVec2::new(0, rng.gen_range(1..height) as i32),
        }
    } else {
        IVec2::new(rng.gen_range(0..30), rng.gen_range(0..30))
    };

    let corridor = Corridor {
        shape: IShape {
            orientation,
            length: rng.gen_range(3..10),
        },
        position,
    };

    let mut corridors = state.layout.corridors.clone();

    let disjoint_from_corridors = corridors.iter().all(|c| !c.collides_with(&corridor));

    if disjoint_from_corridors {
        corridors.push(corridor);
        return Ok(DungeonState {
            layout: DungeonLayout {
                rooms: state.layout.rooms.clone(),
                corridors,
            },
            rng: Rc::clone(&state.rng),
        });
    }

    Err("Failed to add corridor".to_string())
}

#[cfg(test)]
mod dungeon_builder_tests {
    use super::*;

    #[test]
    fn add_room_works() {
        let builder = DungeonGenerator::new().add_step(add_room);

        assert_eq!(builder.generate().unwrap().rooms.len(), 1);
    }

    #[test]
    fn add_two_rooms() {
        let builder = DungeonGenerator::new()
            .add_step(add_room)
            .add_step(add_room);

        assert_eq!(builder.generate().unwrap().rooms.len(), 2);
    }

    #[test]
    fn add_multiple_rooms() {
        let builder = DungeonGenerator::new()
            .add_step(add_room)
            .add_step(add_room)
            .add_step(add_room);

        assert_eq!(builder.generate().unwrap().rooms.len(), 3);
    }

    #[test]
    fn add_retryable_step() {
        let builder = DungeonGenerator::new()
            .add_retryable_step(add_room)
            .add_retryable_step(add_room);

        assert_eq!(builder.generate().unwrap().rooms.len(), 2);
    }

    #[test]
    fn add_retryable_step_5_times() {
        let builder = DungeonGenerator::new()
            .add_retryable_step(add_room)
            .add_retryable_step(add_room)
            .add_retryable_step(add_room)
            .add_retryable_step(add_room)
            .add_retryable_step(add_room);

        let dungeon = builder.generate().unwrap();

        assert_eq!(dungeon.rooms.len(), 5);
    }

    #[test]
    fn add_corridor_works() {
        let builder = DungeonGenerator::new().add_step(add_corridor);

        assert_eq!(builder.generate().unwrap().rooms.len(), 0);
        assert_eq!(builder.generate().unwrap().corridors.len(), 1);
    }

    #[test]
    fn add_room_then_corridor() {
        let builder = DungeonGenerator::new()
            .add_step(add_room)
            .add_step(add_corridor);

        assert_eq!(builder.generate().unwrap().rooms.len(), 1);
        assert_eq!(builder.generate().unwrap().corridors.len(), 1);
    }
}
