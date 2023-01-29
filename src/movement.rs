use crate::map::{TileMap, TileType};
use bevy::prelude::*;

#[derive(Component)]
pub struct Movement {
    pub direction: Direction,
    pub position: Position,
}

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Controllable;

pub enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

pub fn move_entities(
    mut query: Query<(&mut Movement, &mut Transform), Without<Camera2d>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    let mut camera_transform = camera_query.get_single_mut().unwrap();

    for (mut movement, mut transform) in &mut query {
        match movement.direction {
            Direction::Up => {
                camera_transform.translation += Vec3::Y * 16.0;
                transform.translation += Vec3::Y * 16.0;
                movement.position.y += 1;
            }
            Direction::Down => {
                camera_transform.translation -= Vec3::Y * 16.0;
                transform.translation -= Vec3::Y * 16.0;
                movement.position.y -= 1;
            }
            Direction::Left => {
                camera_transform.translation -= Vec3::X * 16.0;
                transform.translation -= Vec3::X * 16.0;
                movement.position.x -= 1;
            }
            Direction::Right => {
                camera_transform.translation += Vec3::X * 16.0;
                transform.translation += Vec3::X * 16.0;
                movement.position.x += 1;
            }
            _ => {}
        }

        movement.direction = Direction::None;
    }
}

pub fn check_collisions(mut movement_query: Query<&mut Movement>, world_map: Res<TileMap>) {
    for mut movement in &mut movement_query {
        let mut x = movement.position.x;
        let mut y = movement.position.y;

        match movement.direction {
            Direction::Up => y += 1,
            Direction::Down => y -= 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
            _ => {}
        }

        let tile_type = world_map.get(IVec2::new(x as i32, y as i32));

        if tile_type == TileType::Wall {
            movement.direction = Direction::None;
        }
    }
}

pub fn player_input_system(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut query: Query<&mut Movement, With<Controllable>>,
) {
    if let Ok(mut movement) = query.get_single_mut() {
        if keyboard_input.clear_just_pressed(KeyCode::Up) {
            movement.direction = Direction::Up;
        }

        if keyboard_input.clear_just_pressed(KeyCode::Down) {
            movement.direction = Direction::Down;
        }

        if keyboard_input.clear_just_pressed(KeyCode::Right) {
            movement.direction = Direction::Right;
        }

        if keyboard_input.clear_just_pressed(KeyCode::Left) {
            movement.direction = Direction::Left;
        }
    }
}
