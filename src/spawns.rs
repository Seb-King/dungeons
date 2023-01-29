use bevy::math::IVec2;
use bevy::prelude::{Commands, Component, DespawnRecursiveExt, Entity, Query, With};

#[derive(Component)]
pub struct Spawn {
    pub position: IVec2,
    pub spawned: bool,
}

pub fn remove_spawn_points(mut commands: Commands, spawns_query: Query<Entity, With<Spawn>>) {
    if let Ok(entity) = spawns_query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
