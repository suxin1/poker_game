use crate::prelude::*;
use bevy::ecs::system::IntoObserverSystem;
use state::cards::{Card, CardValue, Suit};
use crate::game::assets::CardAssets;

const CARD_WIDTH:Val = Vw(5.5);
const CARD_HEIGHT:Val = Vw(8.);

/// 一张牌的图片尺寸固定为 352 x 512（ 11: 16 ）
/// 可选：
///     固定大小 Px(55) x Px(80)
///     自适应 Per(5.5) x Per(8)
pub fn card_view<E, B, M, I>(card_img: Handle<Image>, action: I) -> impl Bundle
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
        Patch(|entity| {
            entity.observe(action);
        })
    )
}

pub fn hands_view () -> impl Bundle {
    (
        Node {
            width: Val::Vw(80.),
            height: CARD_HEIGHT,
            position_type: PositionType::Absolute,
            bottom: Val::Px(16.),
            ..default()
        },
    )
}

pub fn get_card_img(card: Card, card_assets: &CardAssets) -> Handle<Image> {
    match (card.value, card.suit) {
        (CardValue::Ace, Suit::Spades) => card_assets.SA.clone(),
        (CardValue::Ace, Suit::Hearts) => card_assets.HA.clone(),
        (CardValue::Ace, Suit::Diamonds) => card_assets.DA.clone(),
        (CardValue::Ace, Suit::Clubs) => card_assets.CA.clone(),

        (CardValue::Two, Suit::Spades) => card_assets.S2.clone(),
        (CardValue::Two, Suit::Hearts) => card_assets.H2.clone(),
        (CardValue::Two, Suit::Diamonds) => card_assets.D2.clone(),
        (CardValue::Two, Suit::Clubs) => card_assets.C2.clone(),

        (CardValue::Three, Suit::Spades) => card_assets.S3.clone(),
        (CardValue::Three, Suit::Hearts) => card_assets.H3.clone(),
        (CardValue::Three, Suit::Diamonds) => card_assets.D3.clone(),
        (CardValue::Three, Suit::Clubs) => card_assets.C3.clone(),

        (CardValue::Four, Suit::Spades) => card_assets.S4.clone(),
        (CardValue::Four, Suit::Hearts) => card_assets.H4.clone(),
        (CardValue::Four, Suit::Diamonds) => card_assets.D4.clone(),
        (CardValue::Four, Suit::Clubs) => card_assets.C4.clone(),

        (CardValue::Five, Suit::Spades) => card_assets.S5.clone(),
        (CardValue::Five, Suit::Hearts) => card_assets.H5.clone(),
        (CardValue::Five, Suit::Diamonds) => card_assets.D5.clone(),
        (CardValue::Five, Suit::Clubs) => card_assets.C5.clone(),

        (CardValue::Six, Suit::Spades) => card_assets.S6.clone(),
        (CardValue::Six, Suit::Hearts) => card_assets.H6.clone(),
        (CardValue::Six, Suit::Diamonds) => card_assets.D6.clone(),
        (CardValue::Six, Suit::Clubs) => card_assets.C6.clone(),

        (CardValue::Seven, Suit::Spades) => card_assets.S7.clone(),
        (CardValue::Seven, Suit::Hearts) => card_assets.H7.clone(),
        (CardValue::Seven, Suit::Diamonds) => card_assets.D7.clone(),
        (CardValue::Seven, Suit::Clubs) => card_assets.C7.clone(),

        (CardValue::Eight, Suit::Spades) => card_assets.S8.clone(),
        (CardValue::Eight, Suit::Hearts) => card_assets.H8.clone(),
        (CardValue::Eight, Suit::Diamonds) => card_assets.D8.clone(),
        (CardValue::Eight, Suit::Clubs) => card_assets.C8.clone(),

        (CardValue::Nine, Suit::Spades) => card_assets.S9.clone(),
        (CardValue::Nine, Suit::Hearts) => card_assets.H9.clone(),
        (CardValue::Nine, Suit::Diamonds) => card_assets.D9.clone(),
        (CardValue::Nine, Suit::Clubs) => card_assets.C9.clone(),

        (CardValue::Ten, Suit::Spades) => card_assets.S10.clone(),
        (CardValue::Ten, Suit::Hearts) => card_assets.H10.clone(),
        (CardValue::Ten, Suit::Diamonds) => card_assets.D10.clone(),
        (CardValue::Ten, Suit::Clubs) => card_assets.C10.clone(),

        (CardValue::Jack, Suit::Spades) => card_assets.SJ.clone(),
        (CardValue::Jack, Suit::Hearts) => card_assets.HJ.clone(),
        (CardValue::Jack, Suit::Diamonds) => card_assets.DJ.clone(),
        (CardValue::Jack, Suit::Clubs) => card_assets.CJ.clone(),

        (CardValue::Queen, Suit::Spades) => card_assets.SQ.clone(),
        (CardValue::Queen, Suit::Hearts) => card_assets.HQ.clone(),
        (CardValue::Queen, Suit::Diamonds) => card_assets.DQ.clone(),
        (CardValue::Queen, Suit::Clubs) => card_assets.CQ.clone(),

        (CardValue::King, Suit::Spades) => card_assets.SK.clone(),
        (CardValue::King, Suit::Hearts) => card_assets.HK.clone(),
        (CardValue::King, Suit::Diamonds) => card_assets.DK.clone(),
        (CardValue::King, Suit::Clubs) => card_assets.CK.clone(),
    }
}