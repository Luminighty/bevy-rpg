use bevy::prelude::{Plugin, AddAsset};

mod map_asset;

pub struct AssetLoaderPlugin;

pub use map_asset::*;


impl Plugin for AssetLoaderPlugin {
	fn build(&self, app: &mut bevy::prelude::App) {
		app.add_asset::<map_asset::Map>()
			.init_asset_loader::<map_asset::MapAssetLoader>();
	}
}