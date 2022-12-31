mod map;
mod map_generator;
mod movement;
mod player;
mod input;

use bevy::{prelude::*, time::FixedTimestep, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin}};
use bevy_ecs_tilemap::prelude::*;
use movement::{move_entities, player_input_system};

const TIME_STEP: f32 = 1.0 / 60.0;

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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
        .add_system(player_input_system)
        .add_system_set(
            SystemSet::new()
                .with_system(
                    movement::check_collisions
                        .after(player_input_system)
                        .before(move_entities),
                )
                .with_system(movement::move_entities)
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64)),
        )
        .run();
}
