use bevy::{ecs::schedule::ShouldRun, prelude::*, utils::HashSet};
use bevy_ecs_tilemap::prelude::*;

use crate::map_generator::{MapGenerator, TileMap, TileType};

const CHUNK_SIZE: UVec2 = UVec2 { x: 8, y: 8 };

#[derive(Default, Debug, Resource)]
pub struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}

#[derive(Component, Debug)]
pub struct MapSpawner {
    pub respawn_map: bool,
}

pub fn respawn_map_input_system(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut query: Query<&mut MapSpawner>,
) {
    let mut spawner = query.single_mut();

    if keyboard_input.clear_just_pressed(KeyCode::Space) {
        spawner.respawn_map = true;
    }
}

pub fn despawn_map(
    mut spawner_query: Query<&mut MapSpawner>,
    chunks_query: Query<Entity, With<TilemapId>>,
    tilemap_query: Query<Entity, With<TileStorage>>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut commands: Commands,
) {
    let mut map_spawner = spawner_query.single_mut();

    despawn_all_chunks(chunks_query, tilemap_query, chunk_manager, commands);
    map_spawner.respawn_map = false;
}

pub fn despawn_all_chunks(
    chunks_query: Query<Entity, With<TilemapId>>,
    tilemap_query: Query<Entity, With<TileStorage>>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut commands: Commands,
) {
    let manager = chunk_manager.as_mut();
    *manager = ChunkManager::default();
    // for value in chunk_manager.as_mut().spawned_chunks.iter() {
    //     chunk_manager.spawned_chunks.remove(value);
    // }
    for entity in chunks_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in tilemap_query.iter() {
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
    commands.spawn(MapSpawner { respawn_map: false });
}

pub fn spawn_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut chunk_manager: ResMut<ChunkManager>,
) {
    let mut generator = MapGenerator::new();
    let dungeon_map = generator.generate_random();

    let tile_map = dungeon_map.get_tile_map();
    commands.insert_resource(dungeon_map.get_tile_map());

    for x in (-2)..(11) as i32 {
        for y in (-2)..(6) as i32 {
            let chunk_pos = IVec2::new(x * (CHUNK_SIZE.x as i32), y * (CHUNK_SIZE.y as i32));

            chunk_manager
                .spawned_chunks
                .insert(IVec2::new(chunk_pos.x, chunk_pos.y));
            spawn_chunk(&tile_map, chunk_pos, &mut commands, &asset_server);
        }
    }
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
