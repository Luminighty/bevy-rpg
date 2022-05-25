use std::f32::consts::PI;

use bevy::prelude::*;
use crate::{utils::assets::{Map, GID_NONFLAG, FLIPPED_VERTICALLY_FLAG, FLIPPED_HORIZONTALLY_FLAG, MapLayer, FLIPPED_DIAGONALLY_FLAG}, components::MapTile};

use super::MapAssets;


pub fn load_level(
	mut commands: Commands,
	map_assets: Res<MapAssets>,
	levels: Res<Assets<Map>>,
) {
	info!("load level");
	let level = levels.get(map_assets.level.clone()).unwrap();

	for layer in &level.layers {
		let builder = MapBuilder::from(level, layer);
		for (index, data) in layer.data.iter().enumerate() {
			let sprite = builder.create_spritesheet(*data, index, map_assets.texture_atlas.clone());
			commands.spawn_bundle(sprite)
				.insert(MapTile);
		}
	}
}

struct MapBuilder {
	x: usize,
	y: usize,
	visible: bool,
	width: usize,
	height: usize,
	tilewidth: usize,
	tileheight: usize,
}

impl MapBuilder {
	pub fn from(map: &Map, layer: &MapLayer) -> Self {
		Self {
			x: layer.x,
			y: layer.y,
			visible: layer.visible,
			width: layer.width,
			height: layer.height,
			tilewidth: map.tilewidth,
			tileheight: map.tileheight,
		}
	}

	pub fn create_spritesheet(&self, gid: usize, index: usize, tileset: Handle<TextureAtlas>) -> SpriteSheetBundle {
		let sprite_index = (gid & GID_NONFLAG) - 1;
		let mut flip_vertical = (gid & FLIPPED_VERTICALLY_FLAG) != 0;
		let mut flip_horizontal = (gid & FLIPPED_HORIZONTALLY_FLAG) != 0;
		let mut flip_diagonal = (gid & FLIPPED_DIAGONALLY_FLAG) != 0;

		let x = self.x + (index % self.width) * self.tilewidth;
		let y = self.y + (index / self.width) * self.tileheight;
		info!("(x: {} y: {} index: {} flipX: {} flipY: {} flipD: {})", x, y, sprite_index, flip_horizontal, flip_vertical, flip_diagonal);
		let mut transform = Transform::from_xyz(x as f32, -(y as f32), 0.);
		/*transform.rotation = match (flip_horizontal, flip_vertical, flip_diagonal) {
			(false, false, true) => {
				flip_vertical = true;
				Quat::from_rotation_z(90.)
			}
			(true, false, true) => {

			}
			_ => Quat::IDENTITY,
		};*/
		if flip_diagonal {
			flip_horizontal = !flip_horizontal;
			transform.rotation = Quat::from_rotation_z(270.);
		}

		SpriteSheetBundle {
			texture_atlas: tileset,
			transform,
			sprite: TextureAtlasSprite {
				index: sprite_index,
				flip_x: flip_horizontal,
				flip_y: flip_vertical,
				..default()
			},
			..default()	
		}
	}
}
