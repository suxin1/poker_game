use bevy::asset::load_internal_binary_asset;
use bevy::asset::weak_handle;

use crate::core::window::WindowRoot;
use crate::prelude::*;

pub const HAN_FONT_HANDLE: Handle<Font> = weak_handle!("30a293a3dde520f363836e0c7aacdf40");

pub(super) fn plugin(app: &mut App) {
    load_internal_binary_asset!(
        app,
        HAN_FONT_HANDLE,
        "../../assets/font/SmileySans-Oblique.ttf",
        |bytes: &[u8], _path: String| Font::try_from_bytes(bytes.to_vec()).unwrap()
    );

    app.configure::<DynamicFontSize>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DynamicFontSize {
    pub size: Val,
    pub step: f32,
    pub minimum: f32,
}

impl Configure for DynamicFontSize {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_dynamic_font_size.in_set(AppSystems::SyncLate),
        );
    }
}

impl DynamicFontSize {
    pub fn new(size: Val) -> Self {
        Self {
            size,
            step: 0.0,
            minimum: 8.0,
        }
    }

    pub fn with_step(mut self, step: f32) -> Self {
        self.step = step;
        self.minimum = self.minimum.max(step);
        self
    }

    pub fn with_minimum(mut self, minimum: f32) -> Self {
        self.minimum = minimum;
        self
    }
}

#[cfg_attr(feature = "native_dev", hot)]
pub fn apply_dynamic_font_size(
    window_root: Res<WindowRoot>,
    window_query: Query<&Window>,
    mut text_query: Query<(&DynamicFontSize, &ComputedNode, &mut TextFont)>,
) {
    let window = rq!(window_query.get(window_root.primary));
    let viewport_size = window.resolution.size();

    for (font_size, computed_node, mut text) in &mut text_query {
        // Compute font size.
        let size = c!(font_size
            .size
            .resolve(computed_node.size().x, viewport_size));

        // Round down to the nearest multiple of step.
        let resolved = if font_size.step > 0.0 {
            (size / font_size.step).floor() * font_size.step
        } else {
            size
        };
        // Clamp above minimum.
        let size = resolved.max(font_size.minimum);



        // for section in &mut text.sections {
        //     section.style.font_size = size;
        // }
        text.font_size = size;
    }
}