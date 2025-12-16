mod camera;
mod map;
mod player;

use bevy::app::{PluginGroup, PluginGroupBuilder};

/// Plugin group containing the core gameplay logic for Faultline.
///
/// This plugin group should be added to the application after the
/// `DefaultPlugins` group.
pub struct FaultlinePlugins;

impl PluginGroup for FaultlinePlugins {
	fn build(self) -> PluginGroupBuilder {
		PluginGroupBuilder::start::<Self>()
			.add(camera::plugin)
			.add(map::plugin)
			.add(player::plugin)
			.build()
	}
}
