use crate::dungeon_generation::dungeon_generator::SpawnType;
use crate::dungeon_generation::dungeon_generator::{
    add_corridor_then_room, add_room, DungeonGenerator, DungeonLayout,
};
use crate::dungeon_generation::spawn_generation::place_player_spawn;
use crate::player::Player;
use crate::spawns::Spawn;
use crate::{movement, SCREEN_HEIGHT, SCREEN_WIDTH};
use bevy::utils::HashMap;
use bevy::{ecs::schedule::ShouldRun, prelude::*, utils::HashSet};
use bevy_ecs_tilemap::prelude::*;

const CHUNK_SIZE: UVec2 = UVec2 { x: 8, y: 8 };

#[derive(Component, Clone, PartialEq, Copy, Debug)]
pub enum TileType {
    Void,
    Floor,
    Wall,
}

#[derive(Resource, Default)]
pub struct TileMap {
    tile_map: HashMap<IVec2, TileType>,
}

impl TileMap {
    pub fn get(&self, pos: IVec2) -> TileType {
        let tile_option = self.tile_map.get(&pos);
        if let Some(tile) = tile_option {
            return *tile;
        }

        return TileType::Void;
    }

    pub fn set(&mut self, pos: IVec2, tile_type: TileType) {
        self.tile_map.insert(pos, tile_type);
    }
}

#[derive(Default, Debug, Resource)]
pub struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}

#[derive(Component, Debug)]
pub struct MapSpawner {
    pub respawn_map: bool,
    pub generate_next_step: bool,
}

pub fn respawn_map_input_system(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut query: Query<&mut MapSpawner>,
) {
    let mut spawner = query.single_mut();

    if keyboard_input.clear_just_pressed(KeyCode::Space) {
        spawner.generate_next_step = true;
    }

    if keyboard_input.clear_just_pressed(KeyCode::Back) {
        spawner.respawn_map = true;
    }
}

fn get_tile_map(layout: &DungeonLayout) -> TileMap {
    let mut tile_map = TileMap::default();

    for room in &layout.rooms {
        for y in 0..room.shape.height {
            for x in 0..room.shape.width {
                let on_border =
                    y == 0 || y == room.shape.height - 1 || x == 0 || x == room.shape.width - 1;
                let pos = IVec2::new(x as i32 + room.position.x, y as i32 + room.position.y);
                if on_border {
                    tile_map.set(pos, TileType::Wall);
                } else {
                    tile_map.set(pos, TileType::Floor);
                }
            }
        }
    }

    for corridor in &layout.corridors {
        let pos = &corridor.position;
        let shape = &corridor.shape;

        let x_offset = pos.x;
        let y_offset = pos.y;

        let length = shape.length;

        let dir: IVec2 = shape.orientation.into();

        let perp1 = dir.perp();

        for i in 0..length {
            let x = x_offset + (i as i32) * dir.x;
            let y = y_offset + (i as i32) * dir.y;

            let floor_pos = IVec2::new(x, y);
            tile_map.set(floor_pos, TileType::Floor);
            tile_map.set(floor_pos + perp1, TileType::Wall);
            tile_map.set(floor_pos - perp1, TileType::Wall);
        }
    }

    return tile_map;
}

pub fn spawn_chunks_around_camera(
    mut commands: Commands,
    camera_query: Query<&Transform, With<Camera2d>>,
    tile_map: Res<TileMap>,
    asset_server: Res<AssetServer>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let transform = camera_query.get_single().unwrap();

    let x_offset = transform.translation.x as i32 / CHUNK_SIZE.x as i32 / 16;
    let y_offset = transform.translation.y as i32 / CHUNK_SIZE.y as i32 / 16;

    for x in (-6)..(7) as i32 {
        for y in (-4)..(5) as i32 {
            let chunk_pos = IVec2::new(
                (x_offset + x) * (CHUNK_SIZE.x as i32),
                (y_offset + y) * (CHUNK_SIZE.y as i32),
            );

            let chunk = chunk_manager.spawned_chunks.get(&chunk_pos);

            if chunk.is_none() {
                chunk_manager
                    .spawned_chunks
                    .insert(IVec2::new(chunk_pos.x, chunk_pos.y));
                spawn_chunk(&tile_map, chunk_pos, &mut commands, &asset_server);
            }
        }
    }
}

pub fn despawn_chunks_far_away(
    mut commands: Commands,
    chunks_query: Query<(Entity, &Transform), With<TilemapType>>,
    camera_query: Query<&Transform, With<Camera2d>>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let transform = camera_query.get_single().unwrap();

    for (entity, chunk_transform) in chunks_query.iter() {
        let x_distance = (transform.translation.x - chunk_transform.translation.x) as i32;
        let y_distance = (transform.translation.y - chunk_transform.translation.y) as i32;
        if x_distance.abs() > (SCREEN_WIDTH as i32) || y_distance.abs() > (SCREEN_HEIGHT as i32) {
            let chunk_pos = IVec2::new(
                chunk_transform.translation.x as i32 / 16,
                chunk_transform.translation.y as i32 / 16,
            );

            chunk_manager.spawned_chunks.remove(&chunk_pos);

            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn despawn_map(
    mut spawner_query: Query<&mut MapSpawner>,
    chunks_query: Query<Entity, With<TilemapType>>,
    chunk_manager: ResMut<ChunkManager>,
    commands: Commands,
) {
    let mut map_spawner = spawner_query.single_mut();

    despawn_all_chunks(chunks_query, chunk_manager, commands);
    map_spawner.respawn_map = false;
}

pub fn despawn_all_chunks(
    chunks_query: Query<Entity, With<TilemapType>>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut commands: Commands,
) {
    chunk_manager.spawned_chunks.clear();

    for entity in chunks_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn run_if_map_respawned(spawner_query: Query<&MapSpawner>) -> ShouldRun {
    let map_spawner = spawner_query.single();

    if map_spawner.respawn_map {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub fn create_map_spawner(mut commands: Commands) {
    commands.spawn(MapSpawner {
        respawn_map: false,
        generate_next_step: false,
    });
}

pub fn spawn_map(mut commands: Commands) {
    let generator = DungeonGenerator::new()
        .add_step(add_room)
        .add_retryable_step(place_player_spawn)
        .add_retryable_step(add_corridor_then_room)
        .add_retryable_step(add_corridor_then_room)
        .add_retryable_step(add_corridor_then_room)
        .add_retryable_step(add_corridor_then_room)
        .add_retryable_step(add_corridor_then_room)
        .add_retryable_step(add_corridor_then_room)
        .add_retryable_step(add_corridor_then_room)
        .add_retryable_step(add_corridor_then_room)
        .add_retryable_step(add_corridor_then_room);

    let dungeon = generator.generate().unwrap();

    let tile_map = get_tile_map(&dungeon.layout);

    for spawn in dungeon.spawns.iter() {
        if spawn.spawn_type == SpawnType::Player {
            commands.spawn((
                Player,
                Spawn {
                    position: spawn.position,
                },
            ));

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

    commands.insert_resource(tile_map);
}

fn spawn_chunk(
    map: &TileMap,
    chunk_pos: IVec2,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let texture_handle: Handle<Image> = asset_server.load("dungeon-tiles.png");

    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());

    for x in 0..CHUNK_SIZE.x {
        for y in 0..CHUNK_SIZE.y {
            let tile_coords = IVec2::new(x as i32 + chunk_pos.x, y as i32 + chunk_pos.y);
            let tile_type: TileType = map.get(tile_coords);

            let index = get_tile_index(tile_type);

            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        texture_index: TileTextureIndex(index),
                        tilemap_id: TilemapId(tilemap_entity),
                        ..Default::default()
                    },
                    tile_type,
                ))
                .id();
            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    let transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * 16.0,
        chunk_pos.y as f32 * 16.0,
        0.0,
    ));

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: CHUNK_SIZE.into(),
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform,
        ..Default::default()
    });
}

fn get_tile_index(tile_type: TileType) -> u32 {
    match tile_type {
        TileType::Floor => 1,
        TileType::Wall => 2,
        TileType::Void => 0,
    }
}
