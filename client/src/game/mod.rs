pub mod hidden_card;
mod widget;

mod interaction;
mod assets;

use crate::prelude::*;
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((assets::plugin, interaction::plugin));
}
