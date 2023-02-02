use crate::dungeon_generation::dungeon_generator::{Spawn, SpawnType};
use crate::dungeon_generation::dungeon_state::{DungeonLayout, DungeonState};
use bevy::prelude::{Component, IVec2};
use rand::Rng;
use std::rc::Rc;

#[derive(Component, Debug, Default)]
pub struct Key;

pub fn add_key(state: &DungeonState) -> Result<DungeonState, String> {
    let mut rng = state.rng.borrow_mut();

    let index = rng.gen_range(0..state.layout.rooms.len());
    let random_room = state.layout.rooms.get(index);

    if let Some(room) = random_room {
        let x = rng.gen_range(1..(room.shape.width - 1)) as i32;
        let y = rng.gen_range(1..(room.shape.height - 1)) as i32;

        let mut spawns = state.spawns.clone();
        spawns.push(Spawn {
            position: IVec2::new(x, y) + room.position,
            spawn_type: SpawnType::Key,
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

    return Err("Failed to place key".to_string());
}

#[cfg(test)]
mod key_placement_tests {
    use super::*;
    use crate::dungeon_generation::dungeon_generator::{add_room, DungeonGenerator};

    #[test]
    fn fails_if_no_room() {
        let builder = DungeonGenerator::new().add_step(add_key);
        assert_eq!(builder.generate().is_err(), true);
    }

    #[test]
    fn if_at_least_one_room_succeeds() {
        let builder = DungeonGenerator::new().add_step(add_room).add_step(add_key);

        assert_eq!(builder.generate().is_ok(), true);
    }
}
