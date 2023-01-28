use bevy::{
    input::mouse::MouseMotion,
    prelude::{Camera2d, EventReader, Input, MouseButton, Query, Res, Transform, Vec2, Vec3, With},
};

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
