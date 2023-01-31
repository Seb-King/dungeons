mod camera;
mod dungeon_generation;
mod inventory;
mod map;
mod movement;
mod player;
mod spawns;

use crate::camera::{setup_camera, PostProcessingMaterial};
use crate::inventory::{pickup_items, setup_text, text_update_system, Inventory};
use crate::map::{despawn_chunks_far_away, spawn_chunks_around_camera, spawn_map};
use crate::player::spawn_player;
use crate::spawns::{despawn_objects, remove_spawn_points, spawn_key};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::sprite::Material2dPlugin;
use bevy::window::close_on_esc;
use bevy::{prelude::*, time::FixedTimestep};
use bevy_ecs_tilemap::prelude::*;
use camera::pan_camera;
use iyes_loopless::prelude::AppLooplessFixedTimestepExt;
use map::{
    create_map_spawner, despawn_map, respawn_map_input_system, run_if_map_respawned, ChunkManager,
};
use movement::{move_entities, player_input_system};
use std::time::Duration;

const TIME_STEP: f32 = 1.0 / 60.0;
const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 720;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(Material2dPlugin::<PostProcessingMaterial>::default())
        .add_plugin(TilemapPlugin);

    let setup = SystemSet::new()
        .with_system(setup_camera)
        .with_system(spawn_map)
        .with_system(create_map_spawner)
        .with_system(setup_text);

    app.add_startup_system_set(setup)
        .insert_resource(ChunkManager::default())
        .insert_resource(Inventory::default());

    let input_system = SystemSet::new()
        .with_system(close_on_esc)
        .with_system(player_input_system)
        .with_system(respawn_map_input_system);

    app.add_system_set(input_system);

    let spawning_system = SystemSet::new()
        .with_run_criteria(run_if_map_respawned)
        .with_system(remove_spawn_points)
        .with_system(despawn_objects)
        .with_system(despawn_map)
        .with_system(spawn_map);

    app.add_system_set(spawning_system);

    let logic = SystemSet::new()
        .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
        .with_system(move_entities)
        .with_system(spawn_chunks_around_camera)
        .with_system(despawn_chunks_far_away)
        .with_system(pan_camera)
        .with_system(spawn_player)
        .with_system(spawn_key)
        .with_system(pickup_items)
        .with_system(text_update_system);

    app.add_fixed_timestep(Duration::from_secs_f32(TIME_STEP), "game_logic")
        .add_fixed_timestep_system_set("game_logic", 0, logic);

    app.run();
}
