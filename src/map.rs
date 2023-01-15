use bevy::{prelude::*, utils::HashSet};
use bevy_ecs_tilemap::prelude::*;

use crate::map_generator::{MapGenerator, TileMap, TileType};

const CHUNK_SIZE: UVec2 = UVec2 { x: 4, y: 4 };

#[derive(Default, Debug, Resource)]
pub struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
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

    for x in (-4)..(22) as i32 {
        for y in (-4)..(12) as i32 {
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
