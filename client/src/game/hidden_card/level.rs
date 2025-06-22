use shared::cards::{Card, CardValue, Suit};

use crate::prelude::*;
use crate::screens::ScreenState;

use crate::game::assets::CardAssets;
use crate::game::hidden_card::player::{SeatPosition, seat_click, seats_view};
use crate::game::widget::prelude::*;

#[derive(Component)]
pub struct LevelUiRoot;

pub fn spawn_level(
    mut commands: Commands,
    card_assets: Res<CardAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Name::new("Game UI root"),
        Node::COLUMN_CENTER.full_size(),
        Pickable::IGNORE,
        LevelUiRoot,
        StateScoped(ScreenState::Gameplay),
        BackgroundColor(Color::srgba_u8(28, 119, 92, 255)),
        children![
            hands_view(children![
                card_view(
                    get_card_img(Card::new(Suit::Spades, CardValue::Ace), &*card_assets),
                    card_select
                ),
                card_view(
                    get_card_img(Card::new(Suit::Hearts, CardValue::Ace), &*card_assets),
                    card_select
                ),
            ]),
            seats_view()
        ],
    ));
}

fn card_select(_: Trigger<Pointer<Click>>) {
    info!("Clicked a card!");
}
