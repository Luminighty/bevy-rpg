use bevy::{prelude::*, app::PluginGroupBuilder};

pub mod assetloader;

pub use assetloader as assets;

pub struct UtilPlugins;

impl PluginGroup for UtilPlugins {
	fn build(&mut self, group: &mut PluginGroupBuilder) {
			group.add(assetloader::AssetLoaderPlugin)
			;
	}
}