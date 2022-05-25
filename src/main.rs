use bevy::{prelude::*, window::PresentMode};


mod utils;
pub mod resources;
pub mod systems;
pub mod components;

fn main() {
	let mut app = App::new();
	app.insert_resource(WindowDescriptor {
			title: "RPG Game".to_string(),
			width: 500.,
			height: 500.,
			present_mode: PresentMode::Fifo,
			..default()
		})
		.add_plugins(DefaultPlugins)
		.add_plugins(utils::UtilPlugins)
		.add_plugin(resources::ResourcePlugin)
		.add_plugins(systems::SystemPlugins)
		.run();
}
