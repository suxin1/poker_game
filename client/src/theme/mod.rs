//! Reusable UI widgets & theming.

// Unused utilities may trigger this lints undesirably.
#![allow(dead_code)]

pub mod interaction;
pub mod palette;
mod text;
pub mod widget;
mod grid;

#[allow(unused_imports)]
pub mod prelude {
    // pub use super::color::ThemeColor;
    // pub use super::color::ThemeColorFor;
    // pub use super::color::ThemeColorForText;
    pub use super::text::DynamicFontSize;
    pub use super::{interaction::InteractionPalette, palette as ui_palette, widget};
}

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((interaction::plugin, text::plugin));
}
