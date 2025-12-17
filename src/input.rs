use bevy::prelude::*;
use leafwing_input_manager::Actionlike;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::prelude::{InputMap, VirtualDPad};

pub(super) fn plugin(app: &mut App) {
	app.add_plugins(InputManagerPlugin::<InputAction>::default());
}

/// Represents various actions the player can perform through game controller.
#[derive(Actionlike, Reflect, Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum InputAction {
	Interact,
	#[actionlike(DualAxis)]
	Move,
}

impl InputAction {
	/// Returns the default `InputMap` for the game.
	pub fn default_map() -> InputMap<Self> {
		InputMap::new([(Self::Interact, KeyCode::KeyE)])
			.with_dual_axis(Self::Move, VirtualDPad::wasd())
	}
}
