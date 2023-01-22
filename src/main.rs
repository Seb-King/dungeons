mod dungeon_generator;
mod map;
mod map_generator;
mod movement;
mod player;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    time::FixedTimestep,
};
use bevy_ecs_tilemap::prelude::*;
use map::{
    create_map_spawner, despawn_map, respawn_map_input_system, run_if_map_respawned, ChunkManager,
};
use movement::{move_entities, player_input_system};

const TIME_STEP: f32 = 1.0 / 60.0;

const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 720;

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            (SCREEN_WIDTH / 2) as f32 - 8.0,
            (SCREEN_HEIGHT / 2) as f32 - 8.0,
            999.9,
        ),
        ..default()
    });
}

struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(SetupPlugin)
        .add_system(bevy::window::close_on_esc)
        .add_plugin(TilemapPlugin)
        .add_startup_system(map::spawn_map)
        .add_startup_system(player::add_player)
        .add_startup_system(create_map_spawner)
        .add_system(player_input_system)
        .add_system(respawn_map_input_system)
        .insert_resource(ChunkManager::default())
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
                .with_system(movement::move_entities),
        )
        .run();
}
