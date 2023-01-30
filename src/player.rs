use crate::camera::MainCamera;
use bevy::prelude::*;

use crate::movement;
use crate::spawns::Spawn;

#[derive(Component, Default)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    mut player_query: Query<&mut Spawn, Added<Player>>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    if let Ok(mut spawn) = player_query.get_single_mut() {
        if !spawn.spawned {
            spawn.spawned = true;

            let translation = Vec3::new(
                spawn.position.x as f32 * 16.0,
                spawn.position.y as f32 * 16.0,
                1.0,
            );

            commands.spawn((
                Player,
                movement::Controllable,
                movement::Movement {
                    direction: movement::Direction::None,
                    position: movement::Position {
                        x: spawn.position.x,
                        y: spawn.position.y,
                    },
                },
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.1, 1.0, 1.0),
                        custom_size: Some(Vec2::new(16.0, 16.0)),
                        ..default()
                    },
                    transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::Z, 0.0))
                        .with_translation(translation),
                    ..default()
                },
            ));

            if let Ok(mut camera_transform) = camera_query.get_single_mut() {
                camera_transform.translation = translation;
            }
        }
    }
}
