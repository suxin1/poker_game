use crate::prelude::*;
use bevy::ecs::system::IntoObserverSystem;
use shared::cards::{Card, CardValue, Suit};
use crate::animation::offset::NodeOffset;
use crate::game::assets::CardAssets;
use crate::theme::interaction::InteractionSelected;

const CARD_WIDTH:Val = Vw(5.5);
const CARD_HEIGHT:Val = Vw(8.);

/// 一张牌的图片尺寸固定为 352 x 512（ 11: 16 ）
/// 可选：
///     固定大小 Px(55) x Px(80)
///     自适应 Per(5.5) x Per(8)
pub fn card_view<E, B, M, I>(card: Card, card_img: Handle<Image>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: Sync + IntoObserverSystem<E, B, M>,
{
    (
        Node {
            width: CARD_WIDTH,
            height: CARD_HEIGHT,
            ..default()
        },
        children![
            ImageNode::new(card_img),
        ],
        NodeOffset::default(),
        BoxShadow::default(),
        InteractionPalette {
            selected: BoxShadow::from(ShadowStyle {
                color: Color::srgba(0.1, 0.1, 0.1, 0.5),
                x_offset: Val::Px(5.),
                ..default()
            }),
            ..default()
        },
        InteractionSelected(false),
        InteractionPalette {
            selected: NodeOffset::new(Val::Px(0.0), Val::Px(-25.0)),
            ..default()
        },
        CardData(card),
        Patch(|entity| {
            entity.observe(action);
        })
    )
}

#[derive(Component)]
pub struct CardData(pub Card);