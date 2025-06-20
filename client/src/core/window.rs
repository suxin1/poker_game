use bevy::window::{ExitCondition, WindowResolution};
use bevy::window::PresentMode;
use bevy::window::PrimaryWindow;
use bevy::window::WindowMode;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(WindowPlugin {
        primary_window: Some(Window {
            name: Some("bevy_app".to_string()),
            fit_canvas_to_parent: true,
            visible: true,
            position: WindowPosition::At(IVec2::new(0, 0)),
            resolution: WindowResolution::new(844., 390.),
            ..default()
        }),
        exit_condition: ExitCondition::OnPrimaryClosed,
        ..default()
    });

    app.configure::<(WindowRoot, ConfigHandle<WindowConfig>)>();
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct WindowRoot {
    pub primary: Entity,
}

impl Configure for WindowRoot {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
    }
}

impl FromWorld for WindowRoot {
    fn from_world(world: &mut World) -> Self {
        Self {
            primary: world
                .query_filtered::<Entity, With<PrimaryWindow>>()
                .single(world)
                .unwrap(),
        }
    }
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WindowConfig {
    pub title: String,
    pub window_mode: WindowMode,
    pub present_mode: PresentMode,
}

impl Config for WindowConfig {
    const FILE: &'static str = "window.ron";

    fn on_load(&self, world: &mut World) {
        // mut next_screen: ResMut<NextState<Screen>>
        // r!(world.get_resource_mut::<NextState<_>());
        // r!(world.get_resource_mut::<NextState<WindowState>>()).set(WindowState::Ready);

        // r!(world.get_resource_mut::<NextStateBuffer<_>>()).enable(WindowReady);

        let window_root = r!(world.get_resource::<WindowRoot>());
        let mut window = r!(world.get_mut::<Window>(window_root.primary));
        window.title.clone_from(&self.title);
        window.mode = self.window_mode;
        window.present_mode = self.present_mode;
    }
}
