use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};

use crate::map_generator::{TileMap, TileType};

#[derive(Component)]
pub struct Movement {
    pub direction: Direction,
    pub position: Position,
}

#[derive(Component)]
pub struct Position {
    pub x: u32,
    pub y: u32,
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

pub fn move_entities(mut query: Query<(&mut Movement, &mut Transform)>) {
    for (mut movement, mut transform) in &mut query {
        match movement.direction {
            Direction::Up => {
                transform.translation += Vec3::Y * 16.0;
                movement.position.y += 1;
            }
            Direction::Down => {
                transform.translation -= Vec3::Y * 16.0;
                movement.position.y -= 1;
            }
            Direction::Left => {
                transform.translation -= Vec3::X * 16.0;
                movement.position.x -= 1;
            }
            Direction::Right => {
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
    let mut movement = query.single_mut();

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
