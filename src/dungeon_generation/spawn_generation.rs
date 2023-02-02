use crate::dungeon_generation::dungeon_generator::Spawn;
use crate::dungeon_generation::dungeon_generator::SpawnType::Player;
use crate::dungeon_generation::dungeon_state::{DungeonLayout, DungeonState};
use bevy::math::IVec2;
use rand::Rng;
use std::rc::Rc;

pub fn place_player_spawn(state: &DungeonState) -> Result<DungeonState, String> {
    let first_room = state.layout.rooms.get(0);

    if let Some(room) = first_room {
        let mut rng = state.rng.borrow_mut();

        let x = rng.gen_range(1..(room.shape.width - 1)) as i32;
        let y = rng.gen_range(1..(room.shape.height - 1)) as i32;

        let mut spawns = state.spawns.clone();
        spawns.push(Spawn {
            position: IVec2::new(x, y) + room.position,
            spawn_type: Player,
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

    return Err("Could not place player spawn".to_string());
}

#[cfg(test)]
mod player_spawn_generation_tests {
    use super::*;
    use crate::dungeon_generation::dungeon_generator::{add_room, DungeonGenerator};

    #[test]
    fn fails_if_no_room() {
        let builder = DungeonGenerator::new().add_step(place_player_spawn);
        assert_eq!(builder.generate().is_err(), true);
    }

    #[test]
    fn if_there_is_a_room_places_spawn_correctly() {
        let builder = DungeonGenerator::new()
            .add_step(add_room)
            .add_step(place_player_spawn);

        let dungeon = builder.generate();

        assert_eq!(dungeon.is_ok(), true);
    }
}
