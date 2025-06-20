//! Helper functions for creating common widgets.

use std::borrow::Cow;

use bevy::{
    ecs::{spawn::SpawnWith, system::IntoObserverSystem},
    prelude::*,
    ui::Val::*,
};

use crate::prelude::*;
use crate::theme::interaction::InteractionDisabled;
use crate::theme::text::HAN_FONT_HANDLE;
use crate::theme::{interaction::InteractionPalette, palette::*};

/// A root UI node that fills the window and centers its content.
pub fn ui_root(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            position_type: PositionType::Absolute,
            width: Percent(100.0),
            height: Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Px(20.0),
            ..default()
        },
        // Don't block picking events for other UI roots.
        Pickable::IGNORE,
    )
}

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

/// A large rounded button with text and an action defined as an [`Observer`].
pub fn button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        Vw(3.0),
        action,
        (
            Node {
                width: Vw(30.0),
                height: Vw(7.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderRadius::MAX,
        ),
    )
}

/// A small square button with text and an action defined as an [`Observer`].
pub fn button_small<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        Vw(3.0),
        action,
        (
            Node {
                width: Vw(3.0),
                height: Vw(3.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderRadius::MAX,
        ),
    )
}

/// A simple button with text and an action defined as an [`Observer`]. The button's layout is provided by `button_bundle`.
fn button_base<E, B, M, I>(
    text: impl Into<String>,
    font_size: Val,
    action: I,
    button_bundle: impl Bundle,
) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let text = text.into();
    let action = IntoObserverSystem::into_system(action);
    (
        Name::new("Button Inner"),
        Button,
        button_bundle,
        BackgroundColor(BUTTON_BACKGROUND),
        InteractionPalette {
            none: BackgroundColor(ThemeColor::PRIMARY),
            hovered: BackgroundColor(ThemeColor::PRIMARY_HOVERED),
            pressed: BackgroundColor(ThemeColor::PRIMARY_PRESSED),
            disabled: BackgroundColor(ThemeColor::PRIMARY_DISABLED),
        },
        children![(
            text_base(text, font_size, ThemeColor::PRIMARY_TEXT),
            // Don't bubble picking events from the text up to the button.
            Pickable::IGNORE,
        )],
        Patch(|entity| {
            entity.observe(action);
        }),
    )
}

/// Layout
pub fn stretch(children: impl Bundle) -> impl Bundle {
    (
        Name::new("Stretch Layout"),
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_grow: 1.0,
            ..default()
        },
        children,
    )
}

pub fn selector<E1, B1, M1, I1, C, E2, B2, M2, I2>(
    marker: C,
    left_action: I1,
    right_action: I2,
) -> impl Bundle
where
    C: Component,
    E1: Event,
    B1: Bundle,
    I1: Sync + IntoObserverSystem<E1, B1, M1>,
    E2: Event,
    B2: Bundle,
    I2: Sync + IntoObserverSystem<E2, B2, M2>,
{
    (
        Name::new("Selector"),
        Node {
            width: Vw(35.0),
            ..Node::ROW
        },
        marker,
        children![
            (button_small("<", left_action), InteractionDisabled(false)),
            stretch(children![label("")]),
            (button_small(">", right_action), InteractionDisabled(false))
        ],
    )
}
