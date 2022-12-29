use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::map_generator::{Grid, MapGenerator, TileType};

pub fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let map_size = TilemapSize { x: 16, y: 16 };

    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(map_size);

    let generator = MapGenerator::new();
    let tile_map = generator.get_tile_map();

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_type: TileType = tile_map[y as usize][x as usize];

            let index: u32 = match tile_type {
                TileType::Floor => 0,
                TileType::Wall => 1,
            };

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
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}
