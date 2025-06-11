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

    app.configure::<VolumeSelector>();
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
            (
                widget::label("主音量"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                }
            ),
            widget::selector(
                VolumeSelector(VolumeType::Global),
                lower_global_volume,
                raise_global_volume
            ),
            (
                widget::label("界面音量"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                }
            ),
            widget::selector(
                VolumeSelector(VolumeType::Ui),
                lower_ui_volume,
                raise_ui_volume
            ),
            (
                widget::label("音乐音量"),
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                }
            ),
            widget::selector(
                VolumeSelector(VolumeType::Music),
                lower_music_volume,
                raise_music_volume
            ),
        ],
    )
}

// const MIN_VOLUME: f32 = 0.0;
// const MAX_VOLUME: f32 = 3.0;

fn lower_global_volume(_: Trigger<Pointer<Click>>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.master_volume = (audio_settings.master_volume - 0.1).max(0.0);
}

fn raise_global_volume(_: Trigger<Pointer<Click>>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.master_volume = (audio_settings.master_volume + 0.1).min(1.0);
}

fn lower_ui_volume(_: Trigger<Pointer<Click>>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.ui_volume = (audio_settings.ui_volume - 0.1).max(0.0);
}

fn raise_ui_volume(_: Trigger<Pointer<Click>>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.ui_volume = (audio_settings.ui_volume + 0.1).min(1.0);
}

fn lower_music_volume(_: Trigger<Pointer<Click>>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.music_volume = (audio_settings.music_volume - 0.1).max(0.0);
}

fn raise_music_volume(_: Trigger<Pointer<Click>>, mut audio_settings: ResMut<AudioSettings>) {
    audio_settings.music_volume = (audio_settings.music_volume + 0.1).min(1.0);
}

#[derive(Reflect, Debug)]
enum VolumeType {
    Ui,
    Music,
    Global,
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct VolumeSelector(VolumeType);

impl Configure for VolumeSelector {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            update_volume_selector
                .in_set(AppSystems::Update)
                .run_if(in_state(Menu::Settings)),
        );
    }
}

fn update_volume_selector(
    audio_settings: Res<AudioSettings>,
    selector_query: Query<(Entity, &VolumeSelector), With<VolumeSelector>>,
    children_query: Query<&Children>,
    mut text_query: Query<&mut Text>,
    mut disabled_query: Query<&mut InteractionDisabled>,
) {
    for (entity, selector) in &selector_query {
        let volume_type = &selector.0;
        let target_value = match volume_type {
            VolumeType::Ui => audio_settings.ui_volume,
            VolumeType::Music => audio_settings.music_volume,
            VolumeType::Global => audio_settings.master_volume,
        };

        let children = c!(children_query.get(entity))
            .into_iter()
            .collect::<Vec<_>>();

        let left = **c!(children.first());
        c!(disabled_query.get_mut(left)).0 = target_value <= f32::EPSILON; // Disable if volume is 0

        let mid = **c!(children.get(1));
        let mid_children = c!(children_query.get(mid));
        let label = *c!(mid_children.first());
        c!(text_query.get_mut(label)).0 = format!("{:.0}%", target_value * 100.0);

        let right = **c!(children.get(2));
        c!(disabled_query.get_mut(right)).0 = target_value >= 1.0 - f32::EPSILON;
    }
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
