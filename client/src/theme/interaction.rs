use crate::asset_tracking::LoadResource;
use crate::core::audio::{AudioSettings, ui_audio};
use bevy::ecs::component::Mutable;
use bevy::reflect::{GetTypeRegistration, Typed};
use crate::animation::offset::NodeOffset;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionDisabled>();

    // app.register_type::<InteractionPalette>();
    // app.add_systems(Update, apply_interaction_palette);
    app.configure::<InteractionPalette<BackgroundColor>>();
    app.configure::<InteractionPalette<NodeOffset>>();

    app.register_type::<InteractionAssets>();
    app.load_resource::<InteractionAssets>();
    app.add_observer(play_on_hover_sound_effect);
    app.add_observer(play_on_click_sound_effect);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct InteractionDisabled(pub bool);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct InteractionSelected(pub bool);

/// Palette for widget interactions. Add this to an entity that supports
/// [`Interaction`]s, such as a button, to change its [`BackgroundColor`] based
/// on the current interaction shared.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Interaction)]
pub struct InteractionPalette<C: Component<Mutability = Mutable> + Clone> {
    pub none: C,
    pub hovered: C,
    pub pressed: C,
    pub disabled: C,
    pub selected: C,
}

impl<C: Component<Mutability = Mutable> + Clone + Typed + FromReflect + GetTypeRegistration>
    Configure for InteractionPalette<C>
{
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_interaction_palette::<C>.in_set(AppSystems::RecordInput),
        );
    }
}

fn apply_interaction_palette<C: Component<Mutability = Mutable> + Clone>(
    mut palette_query: Query<
        (
            &Interaction,
            &InteractionPalette<C>,
            &mut C,
            Option<&InteractionDisabled>,
            Option<&InteractionSelected>,
        ),
        Or<(Changed<Interaction>, Changed<InteractionDisabled>)>,
    >,
) {
    for (interaction, palette, mut value, disabled, selected) in &mut palette_query {
        *value = if matches!(disabled, Some(InteractionDisabled(true))) {
            &palette.disabled
        } else if matches!(selected, Some(InteractionSelected(true))) {
            // TODO 实体在选中的状态下会阻止 None Hover Pressed 系统交互
            &palette.selected
        } else {
            match interaction {
                Interaction::None => &palette.none,
                Interaction::Hovered => &palette.hovered,
                Interaction::Pressed => &palette.pressed,
            }
        }
        .clone();
        // *value = match interaction {
        //     Interaction::None => &palette.none,
        //     Interaction::Hovered => &palette.hovered,
        //     Interaction::Pressed => &palette.pressed,
        // }
        // .clone();
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct InteractionAssets {
    #[dependency]
    hover: Handle<AudioSource>,
    #[dependency]
    click: Handle<AudioSource>,
}

impl FromWorld for InteractionAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            hover: assets.load("audio/sound_effects/button_hover.ogg"),
            click: assets.load("audio/sound_effects/button_click.ogg"),
        }
    }
}

fn play_on_hover_sound_effect(
    trigger: Trigger<Pointer<Over>>,
    audio_settings: Res<AudioSettings>,
    interaction_assets: Option<Res<InteractionAssets>>,
    interaction_query: Query<Option<&InteractionDisabled>, With<Interaction>>,
    mut commands: Commands,
) {
    let target = trigger.target();
    let interaction_assets = r!(interaction_assets);
    let disabled = rq!(interaction_query.get(target));

    rq!(!matches!(disabled, Some(InteractionDisabled(true))));

    if interaction_query.contains(target) {
        commands.spawn(ui_audio(&audio_settings, interaction_assets.hover.clone()));
    }
}

fn play_on_click_sound_effect(
    trigger: Trigger<Pointer<Click>>,
    audio_settings: Res<AudioSettings>,
    interaction_assets: Option<Res<InteractionAssets>>,
    interaction_query: Query<Option<&InteractionDisabled>, With<Interaction>>,
    mut commands: Commands,
) {
    let target = trigger.target();
    let interaction_assets = r!(interaction_assets);
    let disabled = rq!(interaction_query.get(target));

    rq!(!matches!(disabled, Some(InteractionDisabled(true))));

    if interaction_query.contains(trigger.target()) {
        commands.spawn(ui_audio(&audio_settings, interaction_assets.click.clone()));
    }
}
