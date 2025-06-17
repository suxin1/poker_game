use crate::core::audio::{AudioSettings, ui_audio};
use crate::prelude::*;
use crate::screens::ScreenState;

pub(super) fn plugin(app: &mut App) {
    app.init_collection::<InteractionAssets>();
    app.configure::<InteractionSfx>();
}

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
struct InteractionAssets {
    #[asset(path = "audio/sfx/251390__deadsillyrabbit__button_hover-mp3.ogg")]
    pub sfx_hover: Handle<AudioSource>, // 鼠标悬浮在卡牌上的音效
    #[asset(path = "audio/sfx/253168__suntemple__sfx-ui-button-click.ogg")]
    pub sfx_click: Handle<AudioSource>, // 点击卡牌上的音效
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct InteractionSfx;

impl Configure for InteractionSfx {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_observer(play_hover_sfx);
        app.add_observer(play_click_sfx);
    }
}

fn play_hover_sfx(
    trigger: Trigger<Pointer<Over>>,
    audio_settings: Res<AudioSettings>,
    assets: Option<Res<InteractionAssets>>,
    sfx_query: Query<(), With<InteractionSfx>>,
    mut commands: Commands,
) {
    let assets = r!(assets);
    let target = trigger.target();

    if sfx_query.contains(target) {
        commands.spawn(ui_audio(&audio_settings, assets.sfx_hover.clone()));
    }
}

fn play_click_sfx(
    trigger: Trigger<Pointer<Click>>,
    audio_settings: Res<AudioSettings>,
    assets: Option<Res<InteractionAssets>>,
    sfx_query: Query<(), With<InteractionSfx>>,
    mut commands: Commands,
) {
    let assets = r!(assets);
    let target = trigger.target();
    if sfx_query.contains(target) {
        commands.spawn(ui_audio(&audio_settings, assets.sfx_click.clone()));
    }
}
