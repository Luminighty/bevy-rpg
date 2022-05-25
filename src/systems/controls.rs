use bevy::prelude::*;

use crate::resources::controls::Controls;

pub fn controls_system(mut controls: ResMut<Controls>, keyboard_input: Res<Input<KeyCode>>) {
	match (keyboard_input.pressed(KeyCode::A), keyboard_input.pressed(KeyCode::D)) {
		(true, false) => controls.horizontal = -1.,
		(false, true) => controls.horizontal = 1.,
		_ => controls.horizontal = 0.,
	}

	match (keyboard_input.pressed(KeyCode::W), keyboard_input.pressed(KeyCode::S)) {
		(true, false) => controls.vertical = 1.,
		(false, true) => controls.vertical = -1.,
		_ => controls.vertical = 0.,
	}
}