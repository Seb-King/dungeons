use bevy::{prelude::*, utils::HashSet};
use bevy_ecs_tilemap::prelude::*;

use crate::map_generator::{MapGenerator, TileType};

const CHUNK_SIZE: UVec2 = UVec2 { x: 4, y: 4 };

const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};

#[derive(Default, Debug, Resource)]
struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}

pub fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("dungeon-tiles.png");

    let mut generator = MapGenerator::new();

    let dungeon_map = generator.generate_random();

    let map_size = TilemapSize {
        x: dungeon_map.map_size.0,
        y: dungeon_map.map_size.1,
    };

    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(map_size);

    let tile_map = dungeon_map.get_tile_map();

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let pos = IVec2::new(x as i32, y as i32);
            let tile_type: TileType = tile_map.get(pos);

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
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        ..Default::default()
    });

    commands.insert_resource(dungeon_map.get_tile_map());
}

fn get_tile_index(tile_type: TileType) -> u32 {
    match tile_type {
        TileType::Floor => 1,
        TileType::Wall => 2,
        TileType::Void => 0,
    }
}
