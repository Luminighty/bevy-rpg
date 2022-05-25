use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Collider {
	pub rect: Rect<f32>
}

#[derive(Component)]
pub struct MapTile;