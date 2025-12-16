use bevy::prelude::*;

/// Marker component for the entity used as the primary camera for the game.
///
/// Systems for the camera expect this component to only be present on a single
/// entity.
#[derive(Component, Debug)]
pub struct PrimaryCamera;

/// Marker component for the entity that the camera should focus on.
///
/// Systems for the camera expect this component to only be present on a single
/// entity.
#[derive(Component, Debug)]
pub struct CameraTarget;

const CAMERA_LERP_FACTOR: f32 = 2.8;

/// This plugin contains systems relating to the camera for the game.
///
/// Adding this plugin to the game will handle spawning and interacting with the
/// primary game camera.
pub(super) fn plugin(app: &mut App) {
	app.add_systems(Startup, spawn_camera);
	app.add_systems(Update, follow_camera_target);
}

/// Spawns the primary camera entity.
///
/// This system is run on the `Startup` schedule and will spawn a 2D camera with
/// an orthographic projection matrix to view the game world.
fn spawn_camera(mut commands: Commands) {
	commands.spawn((
		PrimaryCamera,
		Camera2d,
		Projection::Orthographic(OrthographicProjection {
			scale: 0.75,
			..OrthographicProjection::default_2d()
		}),
	));
}

/// Moves the primary camera towards the camera target.
///
/// This system is run on the `Update` schedule and will use linear
/// interpolation to move the transform of the camera towards the transform of
/// the entity with the `CameraTarget` component.
fn follow_camera_target(
	target_transform: Single<&Transform, (With<CameraTarget>, Without<PrimaryCamera>)>,
	mut camera_transform: Single<&mut Transform, (With<PrimaryCamera>, Without<CameraTarget>)>,
	time: Res<Time>,
) {
	let Vec3 { x, y, .. } = target_transform.translation;
	let direction = Vec3::new(x, y, camera_transform.translation.z);

	camera_transform.translation = camera_transform
		.translation
		.lerp(direction, time.delta_secs() * CAMERA_LERP_FACTOR);
}
