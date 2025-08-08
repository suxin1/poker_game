// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev_tools builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod animation;
mod asset_tracking;
// mod audio;
mod core;
mod demo;
mod game;
mod menus;
mod network;
mod prelude;
mod screens;
mod theme;
mod utils;

#[cfg(feature = "dev")]
mod fake_player;

mod plugin;

mod user;

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
            #[cfg(feature = "dev")]
            fake_player::plugin,
            #[cfg(not(feature = "dev"))]
            user::plugin,
            asset_tracking::plugin,
            demo::plugin,
            network::plugin,
            animation::plugin,
            menus::plugin,
            screens::plugin,
            theme::plugin,
            game::plugin,
        ));

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);
    }
}

#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Name::new("Camera"), MainCamera));
}
