use bevy::prelude::*;

use crate::movement;
use crate::spawns::Spawn;

#[derive(Component, Default)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, player_query: Query<&Spawn, With<Player>>) {
    println!("Spawning player");

    if let Ok(spawn) = player_query.get_single() {
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
                    .with_translation(Vec3::new(
                        spawn.position.x as f32 * 16.0,
                        spawn.position.y as f32 * 16.0,
                        1.0,
                    )),
                ..default()
            },
        ));
    }
}

pub fn despawn_player(
    mut commands: Commands,
    spawns_query: Query<Entity, With<Spawn>>,
    player_sprite_query: Query<Entity, (With<Sprite>, With<Player>)>,
) {
    for entity in spawns_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in player_sprite_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
