pub mod assets;

pub mod audio;
pub mod pause;
pub mod window;

#[cfg(feature = "dev")]
pub mod dev_tools;
mod timer_test;

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
        PopupPlugin,
        pause::plugin,
        timer_test::plugin,
        #[cfg(feature = "dev")]
        dev_tools::plugin,
    ));
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
    /// Update Game State
    UpdateState,
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
                Self::UpdateState,
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
