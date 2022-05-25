use bevy::{prelude::*};

use crate::{resources::controls::Controls};

pub struct TestPlugin;

impl Plugin for TestPlugin {
	fn build(&self, app: &mut App) {
		app.add_system(test_sprites)
		;
	}
}

pub fn test_sprites(
	controls: Res<Controls>,
	mut sprites: Query<&mut Transform, With<TextureAtlasSprite>>
) {
	for mut transform in sprites.iter_mut() {
		transform.translation.x += controls.horizontal;
		transform.translation.y += controls.vertical;
	}
}
