pub mod bincode;
pub mod hidden_card;
mod widget;

mod assets;
mod dev;
mod event;
mod interaction;

use crate::prelude::*;
use crate::screens::ScreenState;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        assets::plugin,
        bincode::plugin,
        interaction::plugin,
        widget::plugin,
        event::plugin,
        hidden_card::plugin,
        #[cfg(feature = "dev")]
        dev::plugin,
    ));
}