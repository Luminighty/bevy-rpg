use bevy::prelude::*;

mod map;
mod controls;
mod test;

pub struct SystemPlugins;
impl PluginGroup for SystemPlugins {
	fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
		group.add(GlobalSystem)
			.add(map::MapSystemPlugin)
		;
	}
}


struct GlobalSystem;
impl Plugin for GlobalSystem {
	fn build(&self, app: &mut App) {
		app.add_state(GameState::MapLoading)
			.add_startup_system(load_camera)
			.add_system(controls::controls_system)
			.add_plugin(test::TestPlugin)
		;
	}
}

fn load_camera(
	mut commands: Commands,
) {
	let mut camera = OrthographicCameraBundle::new_2d();
	camera.transform.scale = Vec3::new(0.25, 0.25, 1.);
    commands.spawn_bundle(camera);
}


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
	MenuLoading,
	Menu,
	
	MapLoading,
	Map,
}
