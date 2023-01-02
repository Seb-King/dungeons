use bevy::prelude::*;

use crate::movement;

#[derive(Component)]
struct Player;

pub fn add_player(mut commands: Commands) {
    let player_x = 1;
    let player_y = 1;

    commands.spawn((
        Player,
        movement::Controllable,
        movement::Movement {
            direction: movement::Direction::None,
            position: movement::Position {
                x: player_x,
                y: player_y,
            },
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 1.0, 1.0),
                custom_size: Some(Vec2::new(
                    (player_x as f32) * 16.0,
                    (player_y as f32) * 16.0,
                )),
                ..default()
            },
            transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::Z, 0.0))
                .with_translation(Vec3::new(16.0, 16.0, 1.0)),
            ..default()
        },
    ));
}
