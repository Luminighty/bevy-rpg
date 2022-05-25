use bevy::{
	asset::{AssetLoader, LoadContext, LoadedAsset},
	prelude::*,
	reflect::TypeUuid,
	utils::BoxedFuture,
};
use json;


pub const FLIPPED_HORIZONTALLY_FLAG: usize = 0x80000000;
pub const FLIPPED_VERTICALLY_FLAG: usize = 0x40000000;
pub const FLIPPED_DIAGONALLY_FLAG: usize = 0x20000000;
pub const ROTATED_HEXAGONAL_120_FLAG: usize = 0x10000000;
pub const GID_NONFLAG: usize = !(FLIPPED_HORIZONTALLY_FLAG | FLIPPED_VERTICALLY_FLAG | FLIPPED_DIAGONALLY_FLAG | ROTATED_HEXAGONAL_120_FLAG);




#[derive(Debug, TypeUuid)]
#[uuid = "eae4a5f4-8a33-4c1f-b8df-715a7ec5bfa4"]
pub struct Map {
	pub height: usize,
	pub width: usize,
	pub tileheight: usize,
	pub tilewidth: usize,
	pub layers: Vec<MapLayer>,
}

#[derive(Debug, TypeUuid)]
#[uuid = "dd8adf37-7efe-487d-a9ad-84be18e9cee1"]
pub struct MapLayer {
	pub data: Vec<usize>,
	pub id: usize,
	pub visible: bool,
	pub x: usize,
	pub y: usize,
	pub width: usize,
	pub height: usize,
}

#[derive(Debug, TypeUuid)]
#[uuid = "57fb4e2b-ae1b-4caa-9581-934a70fa1692"]
pub struct MapObject {}

#[derive(Default)]
pub struct MapAssetLoader;

impl AssetLoader for MapAssetLoader {
	fn load<'a>(
		&'a self,
		bytes: &'a [u8],
		load_context: &'a mut LoadContext,
	) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
		Box::pin(async move {
			let data = std::str::from_utf8(bytes)?;
			let data = json::parse(data)?;

			let mut layers: Vec<MapLayer> = Vec::new();
			for layer_data in data["layers"].members() {
				match layer_data["type"].as_str() {
					Some("tilelayer") => layers.push(MapLayer {
						data: layer_data["data"]
							.members()
							.map(|value| value.as_usize().unwrap())
							.collect(),
						id: layer_data["id"].as_usize().unwrap(),
						visible: layer_data["visible"].as_bool().unwrap(),
						x: layer_data["x"].as_usize().unwrap(),
						y: layer_data["y"].as_usize().unwrap(),
						width: layer_data["width"].as_usize().unwrap(),
						height: layer_data["height"].as_usize().unwrap(),
					}),
					_ => {}
				}
			}
			let map = Map {
				height: data["height"].as_usize().unwrap(),
				width: data["width"].as_usize().unwrap(),
				tilewidth: data["tilewidth"].as_usize().unwrap(),
				tileheight: data["tileheight"].as_usize().unwrap(),
				layers,
			};
			info!("Map loaded");
			load_context.set_default_asset(LoadedAsset::new(map));
			Ok(())
		})
	}

	fn extensions(&self) -> &[&str] {
		&["json"]
	}
}
