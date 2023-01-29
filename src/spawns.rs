use bevy::math::IVec2;
use bevy::prelude::Component;

#[derive(Component, Debug)]
pub struct Spawn {
    pub position: IVec2,
}
