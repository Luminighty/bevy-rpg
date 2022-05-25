use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

use crate::utils::assets;
use super::GameState;

mod level;

pub struct MapSystemPlugin;
impl Plugin for MapSystemPlugin {
	fn build(&self, app: &mut bevy::prelude::App) {
		AssetLoader::new(GameState::MapLoading)
			.continue_to_state(GameState::Map)
			.with_collection::<MapAssets>()
			.build(app);
		app.add_system_set(
			SystemSet::on_enter(GameState::Map).with_system(level::load_level)
		)
		.add_system_set(SystemSet::on_exit(GameState::MapLoading).with_system(load_exit))
		.add_system_set(SystemSet::on_enter(GameState::MapLoading).with_system(load_start))
		.add_system_set(SystemSet::on_update(GameState::MapLoading).with_system(load_update))
		;

	}
}

fn load_exit() {
	info!("Load exit");
}

fn load_start() {
	info!("Load start");
}

fn load_update() {
	info!("Load update");
}

//#[derive(AssetCollection)]
pub struct AudioAssets {
	//#[asset(path = "audio/background.ogg")]
	pub background: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct MapAssets {
//	#[asset(key = "level")]
	#[asset(path = "maps/test.json")]
	pub level: Handle<assets::Map>,
//	#[asset(key = "tileset")]
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 32, rows = 32, padding_x = 1., padding_y = 1.))]
    #[asset(path = "colored.png")]
	pub texture_atlas: Handle<TextureAtlas>,
}