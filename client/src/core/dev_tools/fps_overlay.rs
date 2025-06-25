use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
};
use crate::prelude::input_just_pressed;

struct OverlayColor;

impl OverlayColor {
    const RED: Color = Color::srgb(1.0, 0.0, 0.0);
    const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(FpsOverlayPlugin {
        config: FpsOverlayConfig {
            text_config: TextFont {
                // Here we define size of our overlay
                font_size: 32.0,
                // If we want, we can use a custom font
                font: default(),
                // We could also disable font smoothing,
                font_smoothing: FontSmoothing::default(),
                ..default()
            },
            // We can also change color of the overlay
            text_color: OverlayColor::GREEN,
            // We can also set the refresh interval for the FPS counter
            refresh_interval: core::time::Duration::from_millis(100),
            enabled: false,
        },
    });
    app.add_systems(Update, toggle_fps_overlay.run_if(input_just_pressed(TOGGLE_KEY)));
}

const TOGGLE_KEY: KeyCode = KeyCode::Digit2;

fn toggle_fps_overlay(mut overlay_config: ResMut<FpsOverlayConfig>) {
    overlay_config.enabled = !overlay_config.enabled;
}
