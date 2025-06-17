//! The main menu (seen on the title screen).

use bevy::input::touch::Touch;
use bevy::prelude::*;

use crate::theme::interaction::InteractionDisabled;
use crate::{asset_tracking::ResourceHandles, menus::Menu, screens::ScreenState, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Main Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Main),
        // #[cfg(not(target_family = "wasm"))]
        children![
            widget::button("开始", enter_loading_or_gameplay_screen),
            widget::button("设置", open_settings_menu),
            (
                widget::button("退出", exit_app),
                #[cfg(target_family = "wasm")]
                InteractionDisabled(true)
            )
        ],
        // #[cfg(target_family = "wasm")]
        // children![
        //     widget::button("开始", enter_loading_or_gameplay_screen),
        //     widget::button("设置", open_settings_menu),
        // ],
    ));
}

fn enter_loading_or_gameplay_screen(
    _: Trigger<Pointer<Click>>,
    // _: Trigger<Pointer<TouchInput>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<ScreenState>>,
) {
    info!("Entering loading or gameplay screen.");
    if resource_handles.is_all_done() {
        next_screen.set(ScreenState::Gameplay);
    } else {
        next_screen.set(ScreenState::Loading);
    }
}

fn open_settings_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

// #[cfg(not(target_family = "wasm"))]
fn exit_app(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    #[cfg(not(target_family = "wasm"))]
    app_exit.write(AppExit::Success);
}
