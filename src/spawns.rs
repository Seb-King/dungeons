use crate::dungeon_generation::door::Door;
use crate::dungeon_generation::key::Key;
use crate::map::ItemMap;
use crate::movement::Collidable;
use crate::player::Player;
use bevy::math::{IVec2, Quat};
use bevy::prelude::{
    default, Added, Color, Commands, Component, DespawnRecursiveExt, Entity, Or, Query, ResMut,
    Sprite, SpriteBundle, Transform, Vec2, Vec3, With,
};

#[derive(Component)]
pub struct Spawn {
    pub position: IVec2,
    pub spawned: bool,
}

#[derive(Component)]
pub struct Openable {
    pub opened_by: String,
}

pub fn remove_spawn_points(mut commands: Commands, spawns_query: Query<Entity, With<Spawn>>) {
    if let Ok(entity) = spawns_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn spawn_key(
    mut commands: Commands,
    mut key_spawn_query: Query<&mut Spawn, Added<Key>>,
    mut item_map: ResMut<ItemMap>,
) {
    if let Ok(mut spawn) = key_spawn_query.get_single_mut() {
        if !spawn.spawned {
            spawn.spawned = true;

            let translation = Vec3::new(
                spawn.position.x as f32 * 16.0,
                spawn.position.y as f32 * 16.0,
                1.0,
            );

            let entity = commands.spawn((
                Key,
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(1.0, 1.0, 0.4),
                        custom_size: Some(Vec2::new(16.0, 16.0)),
                        ..default()
                    },
                    transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::Z, 0.0))
                        .with_translation(translation),
                    ..default()
                },
            ));

            item_map
                .item_map
                .insert(spawn.position, ("key".to_string(), entity.id()));
        }
    }
}

pub fn spawn_door(mut commands: Commands, mut door_spawn_query: Query<&mut Spawn, Added<Door>>) {
    if let Ok(mut spawn) = door_spawn_query.get_single_mut() {
        if !spawn.spawned {
            spawn.spawned = true;

            let translation = Vec3::new(
                spawn.position.x as f32 * 16.0,
                spawn.position.y as f32 * 16.0,
                1.0,
            );

            commands.spawn((
                Door,
                Openable {
                    opened_by: "key".to_string(),
                },
                Collidable,
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.0, 0.05, 0.1),
                        custom_size: Some(Vec2::new(16.0, 16.0)),
                        ..default()
                    },
                    transform: Transform::from_rotation(Quat::from_axis_angle(Vec3::Z, 0.0))
                        .with_translation(translation),
                    ..default()
                },
            ));
        }
    }
}

pub fn despawn_objects(
    mut commands: Commands,
    player_query: Query<Entity, Or<(With<Player>, With<Key>, With<Door>)>>,
) {
    for entity in player_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
