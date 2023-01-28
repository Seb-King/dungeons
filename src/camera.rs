use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use bevy::prelude::{default, Camera2dBundle, Commands};
use bevy::{
    input::mouse::MouseMotion,
    prelude::{Camera2d, EventReader, Input, MouseButton, Query, Res, Transform, Vec2, Vec3, With},
};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            (SCREEN_WIDTH / 2) as f32 - 8.0,
            (SCREEN_HEIGHT / 2) as f32 - 8.0,
            999.9,
        ),
        ..default()
    });
}

pub fn pan_camera(
    mut ev_motion: EventReader<MouseMotion>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<&mut Transform, With<Camera2d>>,
) {
    let pan_button = MouseButton::Left;

    let mut pan = Vec2::ZERO;

    if input_mouse.pressed(pan_button) {
        for ev in ev_motion.iter() {
            pan += ev.delta;
        }
    }

    for mut transform in query.iter_mut() {
        transform.translation += Vec3::new(-pan.x, pan.y, 0.0);
    }
}
