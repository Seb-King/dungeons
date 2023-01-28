mod camera;
mod dungeon_generation;
mod map;
mod movement;
mod player;

use crate::camera::setup_camera;
use crate::map::{despawn_chunks_far_away, spawn_chunks_around_camera};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::window::close_on_esc;
use bevy::{prelude::*, time::FixedTimestep};
use bevy_ecs_tilemap::prelude::*;
use camera::pan_camera;
use map::{
    create_map_spawner, despawn_map, respawn_map_input_system, run_if_map_respawned, ChunkManager,
};
use movement::{move_entities, player_input_system};

const TIME_STEP: f32 = 1.0 / 60.0;
const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 720;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(TilemapPlugin)
        .add_startup_system_set(
            SystemSet::new()
                .with_system(setup_camera)
                .with_system(map::spawn_map)
                .with_system(player::add_player)
                .with_system(create_map_spawner),
        )
        .add_system_set(
            SystemSet::new()
                .with_system(close_on_esc)
                .with_system(player_input_system)
                .with_system(respawn_map_input_system),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(run_if_map_respawned)
                .with_system(despawn_map.before(map::spawn_map))
                .with_system(map::spawn_map),
        )
        .add_system_set(
            SystemSet::new()
                .with_system(
                    movement::check_collisions
                        .after(player_input_system)
                        .before(move_entities),
                )
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(move_entities)
                .with_system(spawn_chunks_around_camera)
                .with_system(despawn_chunks_far_away)
                .with_system(pan_camera),
        )
        .insert_resource(ChunkManager::default())
        .run();
}
