use bevy::prelude::*;

use crate::movement;

#[derive(Component)]
struct Player;

pub fn add_player(mut commands: Commands) {
    commands.spawn((
        Player,
        movement::Controllable,
        movement::Movement {
            direction: movement::Direction::None,
            position: movement::Position { x: 8, y: 7 },
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.1, 1.0, 1.0),
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::Z, 0.0))
                .with_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        },
    ));
}
