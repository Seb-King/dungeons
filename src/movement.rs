use crate::camera::MainCamera;
use crate::inventory::Inventory;
use crate::map::{TileMap, TileType};
use crate::spawns::Openable;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use bevy::prelude::*;
use std::borrow::BorrowMut;

#[derive(Component)]
pub struct Movement {
    pub direction: Direction,
    pub position: IVec2,
}

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Controllable;

#[derive(Component)]
pub struct Collidable;

#[derive(Clone, Copy)]
pub enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for IVec2 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Right => IVec2::X,
            Direction::Left => IVec2::NEG_X,
            Direction::Up => IVec2::Y,
            Direction::Down => IVec2::NEG_Y,
            Direction::None => IVec2::ZERO,
        }
    }
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
    mut commands: Commands,
    mut query: Query<
        (&mut Movement, &mut Transform),
        (Without<Camera2d>, Without<MainCamera>, Without<Collidable>),
    >,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Collidable>)>,
    collidable_query: Query<(Entity, &Transform, Option<&Openable>), With<Collidable>>,
    world_map: Res<TileMap>,
    mut inventory: ResMut<Inventory>,
) {
    let mut camera_transform = camera_query.get_single_mut().unwrap();

    for (mut movement, mut transform) in &mut query {
        let movement_delta: IVec2 = movement.direction.into();
        let new_pos = movement.position + movement_delta;
        let collides_with_wall = check_if_collides_with_walls(new_pos, &world_map);

        let collides_with_collidable = collides_with_any_collidable(
            commands.borrow_mut(),
            new_pos,
            &collidable_query,
            inventory.borrow_mut(),
        );

        if collides_with_wall || collides_with_collidable {
            movement.direction = Direction::None;
            continue;
        }

        let player_is_near_screen_edge = near_screen_edge(
            transform.translation,
            camera_transform.translation,
            movement.direction,
        );

        let delta_vec3 = Vec3::from((movement_delta.as_vec2(), 0.0));
        movement.position = new_pos;
        transform.translation += delta_vec3 * 16.0;

        if player_is_near_screen_edge {
            camera_transform.translation += delta_vec3 * 16.0;
        }

        movement.direction = Direction::None;
    }
}

fn collides_with_any_collidable(
    commands: &mut Commands,
    pos: IVec2,
    collidable_query: &Query<(Entity, &Transform, Option<&Openable>), With<Collidable>>,
    inventory: &mut ResMut<Inventory>,
) -> bool {
    for (entity, transform, option) in collidable_query.iter() {
        let v: IVec2 = IVec2::new(
            (transform.translation.x as i32) / 16,
            (transform.translation.y as i32) / 16,
        );

        if pos.eq(&v) {
            if let Some(openable) = option {
                let item_count = inventory.get_item_count(&openable.opened_by);
                if item_count > 0 {
                    inventory.remove_item(&openable.opened_by);
                    commands.entity(entity).despawn_recursive();
                    return false;
                }
            }

            return true;
        }
    }

    return false;
}

fn check_if_collides_with_walls(pos: IVec2, map: &TileMap) -> bool {
    return map.get(pos) == TileType::Wall;
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
