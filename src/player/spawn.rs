use bevy::prelude::*;

use crate::camera::CameraTarget;
use crate::player::Player;

pub(super) fn spawn_player(mut commands: Commands) {
	commands.spawn((
		Player,
		CameraTarget,
		Sprite::from_color(Color::WHITE, Vec2::new(32.0, 64.0)),
	));
}
