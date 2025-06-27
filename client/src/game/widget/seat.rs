use crate::game::widget::prelude::*;
use crate::prelude::*;
use bevy::ecs::system::IntoObserverSystem;

const AVATAR_SIZE: Val = Vw(5.5);
const SET_BOX_WIDTH: Val = Vw(7.5);
const SET_BOX_HEIGHT: Val = Vw(9.);

pub struct AbsolutePosition {
    pub bottom: Val,
    pub left: Val,
    pub top: Val,
    pub right: Val,
}

#[derive(Component)]
pub struct PlayerNameText;

#[derive(Component)]
pub struct PlayerAvatarBox;

#[derive(Component)]
pub struct ReadyMarker;

#[derive(Component)]
#[relationship(relationship_target = SeatLinkBy)]
pub struct SeatLink(pub Entity);

#[derive(Component, Deref)]
#[relationship_target(relationship = SeatLink)]
pub struct SeatLinkBy(Vec<Entity>);

pub fn seat_view<E, B, M, I>(
    position: AbsolutePosition,
    marker: impl Bundle,
    color: Color,
    action: I,
) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: Sync + IntoObserverSystem<E, B, M>,
{
    (
        Name::new("Seat"),
        Node {
            width: SET_BOX_WIDTH,
            height: SET_BOX_HEIGHT,
            position_type: PositionType::Absolute,
            bottom: position.bottom,
            left: position.left,
            top: position.top,
            right: position.right,
            justify_content: JustifyContent::SpaceBetween,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::all(Vw(0.5)),
            ..default()
        },
        Pickable {
            should_block_lower: true,
            is_hoverable: true,
        },
        BorderRadius::all(Vw(1.)),
        BackgroundColor(Color::WHITE),
        marker,
        children![
            (
                Node {
                    width: AVATAR_SIZE,
                    height: AVATAR_SIZE,
                    ..default()
                },
                BorderRadius::MAX,
                BackgroundColor(color),
                PlayerAvatarBox,
            ),
            (PlayerNameText, body_text("空"),),
            (ReadyMarker),
        ],
        Patch(|entity| {
            entity.observe(action);
        }),
    )
}

#[derive(Component)]
pub struct ArrowIndicator;

pub fn create_arrow_component(
    node_style: Node,
    texture: Handle<Image>,
    atlas_layout: Handle<TextureAtlasLayout>,
    anim_indices: AnimationIndices,
    rotation: Quat,
) -> impl Bundle {
    (
        Name::new("Arrow Indicator"),
        node_style,
        ImageNode::from_atlas_image(
            texture,
            TextureAtlas {
                layout: atlas_layout,
                index: anim_indices.first,
            },
        ),
        anim_indices,
        ArrowIndicator,
        Transform::from_rotation(rotation),
        Visibility::Hidden,
    )
}
