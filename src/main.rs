use bevy::prelude::*;
use bevy::window::{CursorOptions, ExitCondition};
use faultline::FaultlinePlugins;

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
	let mut app = App::new();

	app.add_plugins(
		DefaultPlugins
			.set(ImagePlugin::default_nearest())
			.set(WindowPlugin {
				primary_window: Some(Window {
					title: format!("{PACKAGE_NAME} {PACKAGE_VERSION}"),
					..Default::default()
				}),
				primary_cursor_options: Some(CursorOptions::default()),
				exit_condition: ExitCondition::OnPrimaryClosed,
				close_when_requested: true,
			}),
	);

	app.add_plugins(FaultlinePlugins);

	app.run();
}
