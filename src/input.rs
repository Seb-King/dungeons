use std::hash::Hash;

use bevy::{prelude::{Resource, KeyCode, Input, Res, ResMut}, utils::HashSet, input::keyboard};


#[derive(Resource)]
pub struct KeyboardInput {
    just_pressed: HashSet<KeyCode>,
    pressed: HashSet<KeyCode>,
    just_released: HashSet<KeyCode>
}

impl KeyboardInput {

}

pub  fn read_keyboard_inputs(mut input_queue: ResMut<KeyboardInput>, keyboard_inputs: Res<Input<KeyCode>>) {
    for pressed_key in keyboard_inputs.get_pressed() {

    }
}