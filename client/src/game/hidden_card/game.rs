use crate::game::assets::CardAssets;
use crate::game::widget::{card_view, get_card_img, hands_view};
use crate::screens::ScreenState;
use shared::cards::{Card, CardValue, Suit};

use crate::prelude::widget::button;
use crate::prelude::*;

pub fn spawn_game(mut commands: Commands, card_assets: Res<CardAssets>) {
    commands.spawn((
        Name::new("Game UI root"),
        Node::COLUMN_CENTER.full_size(),
        Pickable::IGNORE,
        StateScoped(ScreenState::Gameplay),
        children![
            (
                hands_view(),
                children![
                    card_view(
                        get_card_img(Card::new(Suit::Spades, CardValue::Ace), &*card_assets),
                        card_select
                    ),
                    card_view(
                        get_card_img(Card::new(Suit::Hearts, CardValue::Ace), &*card_assets),
                        card_select
                    ),
                ]
            ),
            button("测试", card_select)
        ],
    ));
}

fn card_select(_: Trigger<Pointer<Click>>) {
    println!("Clicked a card!");
}
