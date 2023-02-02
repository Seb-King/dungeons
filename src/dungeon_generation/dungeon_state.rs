use crate::dungeon_generation::room::{Corridor, Room};
use crate::dungeon_generation::spawn_generation::Spawn;
use rand::prelude::ThreadRng;
use std::cell::RefCell;
use std::rc::Rc;

pub struct DungeonState {
    pub layout: DungeonLayout,
    pub spawns: Vec<Spawn>,
    pub(crate) rng: Rc<RefCell<ThreadRng>>,
}

#[derive(Clone, Debug, Default)]
pub struct DungeonLayout {
    pub rooms: Vec<Room>,
    pub corridors: Vec<Corridor>,
}

#[derive(Default)]
pub struct DungeonStateBuilder {
    layout: DungeonLayout,
    spawns: Vec<Spawn>,
    rng: Rc<RefCell<ThreadRng>>,
}

impl DungeonState {
    #[allow(dead_code)]
    fn builder() -> DungeonStateBuilder {
        DungeonStateBuilder::default()
    }
}

impl DungeonStateBuilder {
    pub fn new(
        layout: DungeonLayout,
        spawns: Vec<Spawn>,
        rng: Rc<RefCell<ThreadRng>>,
    ) -> DungeonStateBuilder {
        DungeonStateBuilder {
            layout,
            spawns,
            rng,
        }
    }

    pub fn build(self) -> DungeonState {
        DungeonState {
            layout: self.layout,
            spawns: self.spawns,
            rng: self.rng,
        }
    }

    pub fn from_state(state: &DungeonState) -> DungeonStateBuilder {
        DungeonStateBuilder::new(
            state.layout.clone(),
            state.spawns.clone(),
            Rc::clone(&state.rng),
        )
    }

    #[allow(dead_code)]
    pub fn layout(mut self, layout: DungeonLayout) -> DungeonStateBuilder {
        self.layout = layout;
        self
    }

    #[allow(dead_code)]
    pub fn spawns(mut self, spawns: Vec<Spawn>) -> DungeonStateBuilder {
        self.spawns = spawns;
        self
    }

    #[allow(dead_code)]
    pub fn rng(mut self, rng: Rc<RefCell<ThreadRng>>) -> DungeonStateBuilder {
        self.rng = rng;
        self
    }

    pub fn rooms(mut self, rooms: Vec<Room>) -> DungeonStateBuilder {
        self.layout = DungeonLayout {
            rooms,
            corridors: self.layout.corridors,
        };
        self
    }

    pub fn corridors(mut self, corridors: Vec<Corridor>) -> DungeonStateBuilder {
        self.layout = DungeonLayout {
            rooms: self.layout.rooms,
            corridors,
        };
        self
    }
}
