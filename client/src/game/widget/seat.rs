use crate::prelude::*;
use crate::game::widget::prelude::*;
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

pub fn seat_view<E, B, M, I>(position: AbsolutePosition, marker: impl Bundle, color: Color, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: Sync + IntoObserverSystem<E, B, M>,
{
    (
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
            (
                PlayerNameText,
                body_text("空"),
            ),
            (
                ReadyMarker
            )
        ],
    )
}
