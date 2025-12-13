use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{TiledMap, TilemapAnchor};

const MAP_ASSET_PATH: &str = "map.tmx";

pub(super) fn spawn_map(mut commands: Commands, assets: Res<AssetServer>) {
	let map_file = assets.load(MAP_ASSET_PATH);

	commands.spawn((
		TiledMap(map_file),
		TilemapAnchor::Center,
		Transform::from_xyz(0.0, 0.0, -1.0),
	));
}
