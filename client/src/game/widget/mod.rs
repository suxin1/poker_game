mod card;
mod seat;
mod interaction;


pub mod prelude {
    pub use super::card::*;
    pub use super::seat::*;
}

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
}