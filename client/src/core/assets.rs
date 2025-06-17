use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(AssetPlugin {
        // Wasm builds will check for meta files (that don't exist) if this isn't set.
        // This causes errors and even panics on web build on itch.
        // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
        #[cfg(target_arch = "wasm32")]
        meta_check: bevy::asset::AssetMetaCheck::Never,
        ..default()
    });
}

// pub struct StartupAssets {
//
// }