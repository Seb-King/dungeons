use bevy::prelude::IVec2;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Clone)]
enum Orientation {
    VERTICAL,
    HORIZONTAL,
}

#[derive(Clone)]
struct IShape {
    orientation: Orientation,
    length: u32,
}

#[derive(Clone)]
struct Room {
    shape: Rectangle,
    position: IVec2,
}

#[derive(Clone)]
struct Corridor {
    shape: IShape,
    position: IVec2,
}

struct DungeonState {
    layout: DungeonLayout,
    rng: Rc<RefCell<ThreadRng>>,
}

#[derive(Clone)]
struct DungeonLayout {
    rooms: Vec<Room>,
    corridors: Vec<Corridor>,
}

struct DungeonGenerator {
    steps: Vec<Step<DungeonState>>,
}

trait Generator<T, Params> {
    fn new(params: Params) -> Self;

    fn generate(&self) -> T;
}

type Step<T> = fn(&T) -> T;

impl DungeonGenerator {
    fn new() -> DungeonGenerator {
        DungeonGenerator { steps: Vec::new() }
    }

    fn generate(&self) -> DungeonLayout {
        let layout = DungeonLayout::new();

        let state = DungeonState {
            layout,
            rng: Rc::new(RefCell::new(rand::thread_rng())),
        };

        let new_state: DungeonState = self.steps.iter().fold(state, |s, step| step(&s));

        return new_state.layout;
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

fn add_room(state: &DungeonState) -> DungeonState {
    let mut rng = state.rng.borrow_mut();

    let room = Room {
        shape: Rectangle {
            width: rng.gen_range(6..16),
            height: rng.gen_range(6..16),
        },
        position: IVec2::new(0, 0),
    };

    let mut rooms = state.layout.rooms.clone();
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

        assert_eq!(builder.generate().rooms.len(), 1);
    }

    #[test]
    fn add_multiple_rooms() {
        let builder = DungeonGenerator::new()
            .add_step(add_room)
            .add_step(add_room)
            .add_step(add_room);

        assert_eq!(builder.generate().rooms.len(), 3);
    }
}
