//! The settings menu.
//!
//! Additional settings and accessibility options should go here.

use bevy::{audio::Volume, input::common_conditions::input_just_pressed, prelude::*, ui::Val::*};

use crate::core::audio::AudioSettings;
use crate::prelude::*;
use crate::theme::interaction::InteractionDisabled;
use crate::{menus::Menu, screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Settings), spawn_settings_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Settings).and(input_just_pressed(KeyCode::Escape))),
    );

    // app.register_type::<GlobalVolumeLabel>();
    // app.add_systems(
    //     Update,
    //     update_global_volume_label.run_if(in_state(Menu::Settings)),
    // );
    app.configure::<GlobalVolumeSelector>();
}

fn spawn_settings_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Settings Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Settings),
        children![
            widget::header("Settings"),
            settings_grid(),
            widget::button("Back", go_back_on_click),
        ],
    ));
}

fn settings_grid() -> impl Bundle {
    (
        Name::new("Settings Grid"),
        Node {
            display: Display::Grid,
            row_gap: Vw(1.4),
            column_gap: Vw(6.0),
            grid_template_columns: vec![
                RepeatedGridTrack::flex(1, 1.0),
                RepeatedGridTrack::flex(1, 1.2),
            ],
            ..default()
        },
        children![
            widget::label("主音量"),
            widget::selector(GlobalVolumeSelector, lower_global_volume, raise_global_volume),
            (
                widget::label("Master Volume"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                }
            ),
            global_volume_widget(),
        ],
    )
}

// const MIN_VOLUME: f32 = 0.0;
// const MAX_VOLUME: f32 = 3.0;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct GlobalVolumeSelector;

impl Configure for GlobalVolumeSelector {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            update_global_volume_selector
                .in_set(AppSystems::Update)
                .run_if(in_state(Menu::Settings)),
        );
    }
}

fn update_global_volume_selector(
    audio_settings: Res<AudioSettings>,
    selector_query: Query<Entity, With<GlobalVolumeSelector>>,
    children_query: Query<&Children>,
    mut text_query: Query<&mut Text>,
    mut disabled_query: Query<&mut InteractionDisabled>,
) {
    for entity in &selector_query {
        let children = c!(children_query.get(entity)).into_iter().collect::<Vec<_>>();

        let left = **c!(children.first());
        c!(disabled_query.get_mut(left)).0 = audio_settings.master_volume <= f32::EPSILON; // Disable if volume is 0

        let mid = **c!(children.get(1));
        let mid_children = c!(children_query.get(mid));
        let label = *c!(mid_children.first());
        c!(text_query.get_mut(label)).0 = format!("{:.0}%", audio_settings.master_volume * 100.0);

        let right = **c!(children.get(2));
        c!(disabled_query.get_mut(right)).0 = audio_settings.master_volume >= 1.0 - f32::EPSILON;
    }
}


fn lower_global_volume(_: Trigger<Pointer<Click>>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.master_volume = (audio_settings.master_volume - 0.1).max(0.0);
}

fn raise_global_volume(_: Trigger<Pointer<Click>>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.master_volume = (audio_settings.master_volume + 0.1).min(1.0);
}

fn global_volume_widget() -> impl Bundle {
    (
        Name::new("Global Volume Widget"),
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            widget::button_small("-", lower_global_volume),
            (
                Name::new("Current Volume"),
                Node {
                    padding: UiRect::horizontal(Px(10.0)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(widget::label(""), GlobalVolumeLabel)],
            ),
            widget::button_small("+", raise_global_volume),
        ],
    )
}

// const MIN_VOLUME: f32 = 0.0;
// const MAX_VOLUME: f32 = 3.0;

// fn lower_global_volume(_: Trigger<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
//     let linear = (global_volume.volume.to_linear() - 0.1).max(MIN_VOLUME);
//     global_volume.volume = Volume::Linear(linear);
// }
//
// fn raise_global_volume(_: Trigger<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
//     let linear = (global_volume.volume.to_linear() + 0.1).min(MAX_VOLUME);
//     global_volume.volume = Volume::Linear(linear);
// }

#[derive(Component, Reflect)]
#[reflect(Component)]
struct GlobalVolumeLabel;

fn update_global_volume_label(
    global_volume: Res<GlobalVolume>,
    mut label: Single<&mut Text, With<GlobalVolumeLabel>>,
) {
    let percent = 100.0 * global_volume.volume.to_linear();
    label.0 = format!("{percent:3.0}%");
}

fn go_back_on_click(
    _: Trigger<Pointer<Click>>,
    screen: Res<State<Screen>>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    next_menu.set(if screen.get() == &Screen::Title {
        Menu::Main
    } else {
        Menu::Pause
    });
}

fn go_back(screen: Res<State<Screen>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(if screen.get() == &Screen::Title {
        Menu::Main
    } else {
        Menu::Pause
    });
}
