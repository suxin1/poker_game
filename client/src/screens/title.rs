//! The title screen that appears after the splash screen.

use bevy::prelude::*;

use crate::{menus::Menu, screens::ScreenState};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(ScreenState::Title), open_main_menu);
    app.add_systems(OnExit(ScreenState::Title), close_menu);
}

fn open_main_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
