use bevy::prelude::*;

pub struct Tileset(pub Handle<TextureAtlas>);

impl Tileset {
	pub fn from(texture_atlas: Handle<TextureAtlas>) -> Self {
		Self(texture_atlas)
	}
}

pub enum TilesetId {
	Player = 24,
}