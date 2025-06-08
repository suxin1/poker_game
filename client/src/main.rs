// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod animation;
mod asset_tracking;
// mod audio;
mod demo;
#[cfg(feature = "dev")]
mod dev_tools;
mod menus;
mod screens;
mod theme;
mod core;
mod prelude;
mod utils;

use crate::prelude::*;
fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Core plugins.
        app.add_plugins(core::plugin);

        // Add other plugins.
        app.add_plugins((
            asset_tracking::plugin,
            demo::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            animation::plugin,
            menus::plugin,
            screens::plugin,
            theme::plugin,
        ));

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), Camera2d));
}
