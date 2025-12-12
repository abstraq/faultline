mod camera;
mod player;

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub struct FaultlinePlugins;

impl PluginGroup for FaultlinePlugins {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(camera::plugin)
			.add(player::plugin)
			.build()
	}
}
