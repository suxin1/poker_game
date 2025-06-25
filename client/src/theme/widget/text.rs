use crate::prelude::{default, Bundle, Color, DynamicFontSize, Name, Text, TextColor, TextFont, Val, Vw};
use crate::prelude::ui_palette::{ThemeColor, HEADER_TEXT};
use crate::theme::text::HAN_FONT_HANDLE;

/// A app wide base text widget, all text should use this.
pub fn text_base(text: impl AsRef<str>, font_size: Val, text_color: Color) -> impl Bundle {
    let text = text.as_ref();
    (
        Name::new(format!("Label(\"{text}\")")),
        Text::new(text),
        TextColor(text_color),
        TextFont {
            font: HAN_FONT_HANDLE,
            ..default()
        },
        DynamicFontSize::new(font_size).with_step(8.0),
    )
}

pub fn body_text(text: impl AsRef<str>) -> impl Bundle {
    text_base(text, Vw(2.), ThemeColor::BODY_TEXT_DARK)
}

/// A simple header label. Bigger than [`label`].
pub fn header(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Header"),
        Text(text.into()),
        TextFont::from_font_size(40.0),
        TextColor(HEADER_TEXT),
    )
}

/// A simple text label.
pub fn label(text: impl Into<String>) -> impl Bundle {
    text_base(text.into(), Vw(3.5), ThemeColor::BODY_TEXT_LIGHT)
}