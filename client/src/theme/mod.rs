//! Reusable UI widgets & theming.

// Unused utilities may trigger this lints undesirably.
#![allow(dead_code)]

pub mod interaction;
pub mod palette;
mod text;
pub mod widget;
mod grid;

use crate::prelude::*;

#[allow(unused_imports)]
pub mod prelude {
    pub use super::text::DynamicFontSize;
    pub use super::{interaction::InteractionPalette, palette as ui_palette, widget};
    pub use super::widget::*;
}


pub(super) fn plugin(app: &mut App) {
    app.add_plugins((interaction::plugin, text::plugin));
}
