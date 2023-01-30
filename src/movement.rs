use crate::camera::MainCamera;
use crate::map::{TileMap, TileType};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
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

#[derive(Clone, Copy)]
pub enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

fn near_screen_edge(pos: Vec3, camera_pos: Vec3, direction: Direction) -> bool {
    let screen_pos = pos - camera_pos;

    return match direction {
        Direction::Right => screen_pos.x > (SCREEN_WIDTH / 4) as f32,
        Direction::Left => screen_pos.x < -((SCREEN_WIDTH / 4) as f32),
        Direction::Up => screen_pos.y > (SCREEN_HEIGHT / 4) as f32,
        Direction::Down => screen_pos.y < -((SCREEN_HEIGHT / 4) as f32),
        _ => false,
    };
}

pub fn move_entities(
    mut query: Query<(&mut Movement, &mut Transform), (Without<Camera2d>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    world_map: Res<TileMap>,
) {
    let mut camera_transform = camera_query.get_single_mut().unwrap();

    for (mut movement, mut transform) in &mut query {
        let collides = check_if_collides_with_walls(
            IVec2::new(movement.position.x, movement.position.y),
            &movement.direction,
            &world_map,
        );

        if collides {
            movement.direction = Direction::None;
            continue;
        }

        let player_is_near_screen_edge = near_screen_edge(
            transform.translation,
            camera_transform.translation,
            movement.direction,
        );

        let mut camera_delta = Vec3::ZERO;
        match movement.direction {
            Direction::Up => {
                camera_delta += Vec3::Y * 16.0;
                transform.translation += Vec3::Y * 16.0;
                movement.position.y += 1;
            }
            Direction::Down => {
                camera_delta -= Vec3::Y * 16.0;
                transform.translation -= Vec3::Y * 16.0;
                movement.position.y -= 1;
            }
            Direction::Left => {
                camera_delta -= Vec3::X * 16.0;
                transform.translation -= Vec3::X * 16.0;
                movement.position.x -= 1;
            }
            Direction::Right => {
                camera_delta += Vec3::X * 16.0;
                transform.translation += Vec3::X * 16.0;
                movement.position.x += 1;
            }
            _ => {}
        }

        if player_is_near_screen_edge {
            camera_transform.translation += camera_delta;
        }

        movement.direction = Direction::None;
    }
}

fn check_if_collides_with_walls(pos: IVec2, direction: &Direction, map: &TileMap) -> bool {
    let mut x = pos.x;
    let mut y = pos.y;

    match direction {
        Direction::Up => y += 1,
        Direction::Down => y -= 1,
        Direction::Left => x -= 1,
        Direction::Right => x += 1,
        _ => {}
    }

    let tile_type = map.get(IVec2::new(x as i32, y as i32));

    if tile_type == TileType::Wall {
        return true;
    }

    return false;
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
