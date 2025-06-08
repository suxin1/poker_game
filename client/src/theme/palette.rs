use std::ops::Index;
use bevy::prelude::*;
use crate::prelude::{Deserialize, Serialize};

/// #ddd369
pub const LABEL_TEXT: Color = Color::srgb(0.867, 0.827, 0.412);

/// #fcfbcc
pub const HEADER_TEXT: Color = Color::srgb(0.988, 0.984, 0.800);

/// #ececec
pub const BUTTON_TEXT: Color = Color::srgb(0.925, 0.925, 0.925);
/// #4666bf
pub const BUTTON_BACKGROUND: Color = Color::srgb(0.275, 0.400, 0.750);
/// #6299d1
pub const BUTTON_HOVERED_BACKGROUND: Color = Color::srgb(0.384, 0.600, 0.820);
/// #3d4999
pub const BUTTON_PRESSED_BACKGROUND: Color = Color::srgb(0.239, 0.286, 0.600);

#[derive(Reflect, Serialize, Deserialize)]
pub struct ThemeColorList([Color; 11]);

impl Index<ThemeColor> for ThemeColorList {
    type Output = Color;

    fn index(&self, index: ThemeColor) -> &Self::Output {
        &self.0[index as usize]
    }
}

// impl ThemeColorList {
//     fn get(index: ThemeColor) {
//         Self[index]
//     }
// }


/// See: <https://getbootstrap.com/docs/5.3/customize/color/>.
#[derive(Reflect, Clone, Copy, Default)]
pub enum ThemeColor {
    // Absolute colors.
    #[default]
    White,
    Invisible,

    // Semantic colors.
    Body,
    BodyText,

    Primary,
    PrimaryHovered,
    PrimaryPressed,
    PrimaryDisabled,
    PrimaryText,

    // Other UI colors.
    Popup,
    Overlay,
}

pub const THEME_COLOR_LIST:ThemeColorList = ThemeColorList([
    // White
    Color::srgba(1.0,1.0, 1.0, 1.0),
    // Invisible
    Color::srgba(0.000, 0.000,  0.000,  0.000),

    // Body
    Color::srgba(0.157, 0.157,  0.157,  1.000),
    // BodyText
    Color::srgba(0.925, 0.925,  0.925,  1.000),

    // Primary
    Color::srgba(0.700, 0.400,  0.700,  1.000),
    // PrimaryHovered
    Color::srgba(0.850, 0.500,  0.820,  1.000),
    // PrimaryPressed
    Color::srgba(0.500, 0.300,  0.500,  1.000),
    // PrimaryDisabled
    Color::srgba(0.400, 0.200,  0.400,  1.000),
    // PrimaryText
    Color::srgba(0.157, 0.157,  0.157,  1.000),

    // Popup
    Color::srgba(0.106, 0.118,  0.122,  0.850),
    // Overlay
    Color::srgba(0.157, 0.157,  0.157,  0.980),
]);