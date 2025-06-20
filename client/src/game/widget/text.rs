use crate::prelude::*;
use crate::prelude::ui_palette::ThemeColor;
use crate::prelude::widget::text_base;

pub fn body_text(text: impl Into<String>) -> impl Bundle {
    text_base(text.into(), Vw(2.), ThemeColor::BODY_TEXT_DARK)
}