use crate::dungeon_generation::room::{Collision, Corridor, Rectangle, Room};
use bevy::prelude::IVec2;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

struct DungeonState {
    layout: DungeonLayout,
    rng: Rc<RefCell<ThreadRng>>,
}

#[derive(Clone)]
struct DungeonLayout {
    rooms: Vec<Room>,
    corridors: Vec<Corridor>,
}

pub struct DungeonGenerator {
    steps: Vec<Step<DungeonState>>,
}

trait Generator<T, Params> {
    fn new(params: Params) -> Self;

    fn generate(&self) -> T;
}

type Step<T> = fn(T) -> Result<T, String>;

impl DungeonGenerator {
    pub fn new() -> DungeonGenerator {
        DungeonGenerator { steps: Vec::new() }
    }

    fn generate(&self) -> Result<DungeonLayout, String> {
        let layout = DungeonLayout::new();

        let state = DungeonState {
            layout,
            rng: Rc::new(RefCell::new(rand::thread_rng())),
        };

        let new_state: Result<DungeonState, String> = self
            .steps
            .iter()
            .try_fold(state, |result, step| step(result));

        return new_state.map(|final_state| final_state.layout);
    }

    fn add_step(mut self, step: Step<DungeonState>) -> Self {
        self.steps.push(step);
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

fn add_room(state: DungeonState) -> Result<DungeonState, String> {
    let rooms = state.layout.rooms.clone();

    if rooms.len() == 0 {
        return Ok(add_initial_room(&state));
    }

    let mut rng = state.rng.borrow_mut();

    let room = Room {
        shape: Rectangle {
            width: rng.gen_range(6..16),
            height: rng.gen_range(6..16),
        },
        position: IVec2::new(30, 30),
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

    Err("Failed to add room".into())
}

fn add_initial_room(state: &DungeonState) -> DungeonState {
    let mut rng = state.rng.borrow_mut();
    let mut rooms = state.layout.rooms.clone();

    let room = Room {
        shape: Rectangle {
            width: rng.gen_range(6..16),
            height: rng.gen_range(6..16),
        },
        position: IVec2::new(0, 0),
    };

    rooms.push(room);

    return DungeonState {
        layout: DungeonLayout {
            rooms,
            corridors: state.layout.corridors.clone(),
        },
        rng: Rc::clone(&state.rng),
    };
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
}
