pub mod assets;

pub mod audio;
pub mod pause;
pub mod window;

#[cfg(feature = "dev")]
pub mod dev_tools;

use crate::prelude::*;
use bevy::audio::AudioPlugin;

pub(super) fn plugin(app: &mut App) {
    app.configure::<AppSystems>();

    app.add_plugins(
        DefaultPlugins
            .build()
            .set(ImagePlugin::default_nearest())
            .replace::<AudioPlugin>(audio::plugin)
            .replace::<WindowPlugin>(window::plugin)
            .disable::<AssetPlugin>()
            .add_before::<WindowPlugin>(assets::plugin),
    );

    app.add_plugins((
        pause::plugin,
        // PointerInputPlugin {
        //     is_mouse_enabled: true,
        //     is_touch_enabled: true,
        // },
        #[cfg(feature = "dev")]
        dev_tools::plugin,
    ));

    // #[cfg(target_arch = "wasm32")]
    // app.add_systems(Update, touch_system);
}

#[cfg(target_arch = "wasm32")]
fn touch_system(touches: Res<Touches>) {
    for touch in touches.iter_just_pressed() {
        info!(
      "just pressed touch with id: {:?}, at: {:?}",
      touch.id(),
      touch.position()
    );
    }

    for touch in touches.iter_just_released() {
        info!(
      "just released touch with id: {:?}, at: {:?}",
      touch.id(),
      touch.position()
    );
    }

    for touch in touches.iter_just_canceled() {
        info!("canceled touch with id: {:?}", touch.id());
    }

    // you can also iterate all current touches and retrieve their shared like this:
    for touch in touches.iter() {
        info!("active touch: {:?}", touch);
        info!("  just_pressed: {}", touches.just_pressed(touch.id()));
    }
}


/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
/// Game logic steps for the [`Update`] schedule.
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AppSystems {
    /// Synchronize start-of-frame values.
    SyncEarly,
    /// Tick timers.
    TickTimers,
    /// Record player and AI input.
    RecordInput,
    /// Step game logic.
    Update,
    /// Handle events emitted this frame.
    HandleEvents,
    /// Apply late commands.
    ApplyCommands,
    /// Synchronize end-of-frame values.
    SyncLate,
}

impl Configure for AppSystems {
    fn configure(app: &mut App) {
        app.configure_sets(
            Update,
            (
                Self::SyncEarly,
                Self::TickTimers,
                Self::Update,
                Self::RecordInput,
                Self::HandleEvents,
                Self::ApplyCommands,
                Self::SyncLate,
            )
                .chain(),
        );
    }
}
