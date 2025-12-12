use bevy::prelude::*;

use crate::player::Player;

const DEFAULT_MOVE_SPEED: f32 = 192.0;

pub(super) fn handle_movement_input(
	mut transform: Single<&mut Transform, With<Player>>,
	input: Res<ButtonInput<KeyCode>>,
	time: Res<Time>,
) {
	let mut direction = Vec2::ZERO;

	if input.pressed(KeyCode::KeyW) {
		direction.y += 1.0; // North
	}

	if input.pressed(KeyCode::KeyA) {
		direction.x -= 1.0; // West
	}

	if input.pressed(KeyCode::KeyS) {
		direction.y -= 1.0; // South
	}

	if input.pressed(KeyCode::KeyD) {
		direction.x += 1.0; // East
	}

	if direction != Vec2::ZERO {
		let delta = direction.normalize() * DEFAULT_MOVE_SPEED * time.delta_secs();

		transform.translation.x += delta.x;
		transform.translation.y += delta.y;
	}
}
