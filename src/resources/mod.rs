use bevy::prelude::*;

pub mod controls;
pub mod tileset;
mod level;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<controls::Controls>()
		;
	}
}