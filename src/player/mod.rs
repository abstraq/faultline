mod movement;
mod spawn;

use bevy::prelude::*;

/// Marker component for the player in the game.
#[derive(Component, Debug)]
pub struct Player;

pub(super) fn plugin(app: &mut App) {
	app.add_systems(Startup, spawn::spawn_player);
	app.add_systems(Update, movement::handle_movement_input);
}
