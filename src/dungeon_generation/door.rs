use crate::dungeon_generation::dungeon_generator::{Spawn, SpawnType};
use crate::dungeon_generation::dungeon_state::{DungeonLayout, DungeonState};
use bevy::prelude::{Component, IVec2};
use rand::Rng;
use std::rc::Rc;

#[derive(Component, Debug)]
pub struct Door;

pub fn add_door(state: &DungeonState) -> Result<DungeonState, String> {
    let mut rng = state.rng.borrow_mut();

    let index = rng.gen_range(0..state.layout.corridors.len());
    let random_corridor = state.layout.corridors.get(index);

    if let Some(corridor) = random_corridor {
        let len = rng.gen_range(1..corridor.shape.length) as i32;

        let dir: IVec2 = corridor.shape.orientation.into();

        let mut spawns = state.spawns.clone();
        spawns.push(Spawn {
            position: corridor.position + dir * len,
            spawn_type: SpawnType::Door,
        });

        return Ok(DungeonState {
            layout: DungeonLayout {
                rooms: state.layout.rooms.clone(),
                corridors: state.layout.corridors.clone(),
            },
            spawns,
            rng: Rc::clone(&state.rng),
        });
    }

    return Err("Failed to place door".to_string());
}
