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


/// See: <https://getbootstrap.com/docs/5.3/customize/color/>.
#[derive(Reflect, Serialize, Deserialize)]
pub struct ThemeColor;

impl ThemeColor {
    // Absolute colors
    pub const WHITE: Color = Color::srgba(1.0, 1.0, 1.0, 1.0);
    pub const INVISIBLE: Color = Color::srgba(0.000, 0.000, 0.000, 0.000);

    // Semantic colors
    pub const BODY: Color = Color::srgba(0.157, 0.157, 0.157, 1.000);
    pub const BODY_TEXT_LIGHT: Color = Color::srgba(0.925, 0.925, 0.925, 1.000);
    pub const BODY_TEXT_DARK: Color = Color::srgba(0.1, 0.1, 0.1, 1.000);

    // Primary
    pub const PRIMARY: Color = Color::srgba(0.700, 0.400, 0.700, 1.000);
    pub const PRIMARY_HOVERED: Color = Color::srgba(0.850, 0.500, 0.820, 1.000);
    pub const PRIMARY_PRESSED: Color = Color::srgba(0.500, 0.300, 0.500, 1.000);
    pub const PRIMARY_DISABLED: Color = Color::srgba(0.400, 0.200, 0.400, 1.000);
    pub const PRIMARY_TEXT: Color = Color::srgba(0.157, 0.157, 0.157, 1.000);

    // Other UI colors
    pub const POPUP: Color = Color::srgba(0.106, 0.118, 0.122, 0.850);
    pub const OVERLAY: Color = Color::srgba(0.157, 0.157, 0.157, 0.980);

    // 添加新颜色示例:
    // pub const NEW_COLOR: Color = Color::srgba(0.5, 0.5, 0.5, 1.0);
}