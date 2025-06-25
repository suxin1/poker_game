mod fps_overlay;
mod ui;
mod ui_picking;

use crate::prelude::*;
use crate::screens::ScreenState;
use bevy::dev_tools::states::log_transitions;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, log_transitions::<ScreenState>);

    app.add_plugins((ui::plugin, ui_picking::plugin, fps_overlay::plugin));
}
