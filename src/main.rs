mod map;
mod map_generator;
mod movement;
mod player;

use bevy::{prelude::*, time::FixedTimestep};
use bevy_ecs_tilemap::prelude::*;

#[derive(Component)]
struct Velocity {
    translation: Vec3,
    rotation: f32,
}

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

fn move_system(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    let delta = time.delta_seconds();

    for (velocity, mut transform) in &mut query {
        transform.translation += velocity.translation;
        transform.rotate_z(velocity.rotation * delta);
    }
}

fn player_input_system(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity>) {
    let mut vel = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::Up) {
        vel += Vec3::Y;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        vel += Vec3::NEG_Y;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        vel += Vec3::X;
    }

    if keyboard_input.pressed(KeyCode::Left) {
        vel += Vec3::NEG_X;
    }

    if keyboard_input.pressed(KeyCode::Space) {
        vel = Vec3::ZERO;
    }

    for mut velocity in &mut query {
        if vel == Vec3::ZERO {
            velocity.translation = Vec3::ZERO;
        } else {
            velocity.translation = Vec3::normalize(vel) * 10.0;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SetupPlugin)
        .add_system(bevy::window::close_on_esc)
        .add_plugin(TilemapPlugin)
        .add_startup_system(map::spawn_map)
        .add_startup_system(player::add_player)
        .add_system_set(
            SystemSet::new()
                .with_system(player_input_system.before(move_system))
                .with_system(move_system)
                .with_system(movement::player_input_system.before(movement::move_entities))
                .with_system(
                    movement::check_collisions
                        .after(movement::player_input_system)
                        .before(movement::move_entities),
                )
                .with_system(movement::move_entities)
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64)),
        )
        .run();
}
