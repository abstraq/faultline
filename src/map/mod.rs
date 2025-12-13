mod spawn;

use bevy::prelude::*;
use bevy_ecs_tiled::tiled::TiledPlugin;

pub(super) fn plugin(app: &mut App) {
	app.add_plugins(TiledPlugin::default());
	app.add_systems(Startup, spawn::spawn_map);
}
