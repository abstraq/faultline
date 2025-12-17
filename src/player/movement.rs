use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::input::InputAction;
use crate::player::Player;

const DEFAULT_MOVE_SPEED: f32 = 192.0;

pub(super) fn handle_movement_input(
	mut transform: Single<&mut Transform, With<Player>>,
	input: Single<&ActionState<InputAction>, With<Player>>,
	time: Res<Time>,
) {
	let mut direction = input.axis_pair(&InputAction::Move);

	if direction != Vec2::ZERO {
		let delta = direction.normalize() * DEFAULT_MOVE_SPEED * time.delta_secs();

		transform.translation.x += delta.x;
		transform.translation.y += delta.y;
	}
}
