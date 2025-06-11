mod ui;
mod ui_picking;

use crate::prelude::*;
use crate::screens::Screen;
use bevy::dev_tools::states::log_transitions;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, log_transitions::<Screen>);

    app.add_plugins((ui::plugin, ui_picking::plugin));
}
