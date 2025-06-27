mod card;
mod interaction;
mod seat;

pub mod prelude {
    pub use super::card::*;
    pub use super::seat::{
        AbsolutePosition, ArrowIndicator, PlayerAvatarBox, PlayerNameText, create_arrow_component,
        seat_view,
    };
}

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(interaction::plugin);
}
