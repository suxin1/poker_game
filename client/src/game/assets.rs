use crate::prelude::*;
use shared::cards::{Card, CardValue, Suit};

pub(super) fn plugin(app: &mut App) {
    app.init_collection::<CardAssets>();
    app.init_collection::<CardBackAssets>();
    app.init_collection::<IndicatorAsset>();
    app.init_collection::<SmallCardAssets>();
    app.init_collection::<Icon64Assets>();
}

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct IndicatorAsset {
    #[asset(path = "sprite/arrow-sheet.png")]
    pub arrow: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub(crate) struct SmallCardAssets {
    #[asset(texture_atlas_layout(tile_size_x = 150, tile_size_y = 218, columns = 4, rows = 13))]
    small_card_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "cards/cards_small.png")]
    small_cards: Handle<Image>
}

impl SmallCardAssets {
    pub fn image_node(&self, card: &Card) -> ImageNode {
        let index = self.get_index(card);
        ImageNode::from_atlas_image(
            self.small_cards.clone(),
            TextureAtlas {
                index,
                layout: self.small_card_layout.clone(),
            }
        )
    }

    pub fn get_index(&self, card: &Card) -> usize {
        match (&card.value, &card.suit) {
            (CardValue::Ace, Suit::Clubs) => 0,
            (CardValue::Ace, Suit::Spades) => 1,
            (CardValue::Ace, Suit::Diamonds) => 2,
            (CardValue::Ace, Suit::Hearts) => 3,

            (CardValue::Two, Suit::Clubs) => 4,
            (CardValue::Two, Suit::Spades) => 5,
            (CardValue::Two, Suit::Diamonds) => 6,
            (CardValue::Two, Suit::Hearts) => 7,

            (CardValue::Three, Suit::Clubs) => 8,
            (CardValue::Three, Suit::Spades) => 9,
            (CardValue::Three, Suit::Diamonds) => 10,
            (CardValue::Three, Suit::Hearts) => 11,

            (CardValue::Four, Suit::Clubs) => 12,
            (CardValue::Four, Suit::Spades) => 13,
            (CardValue::Four, Suit::Diamonds) => 14,
            (CardValue::Four, Suit::Hearts) => 15,

            (CardValue::Five, Suit::Clubs) => 16,
            (CardValue::Five, Suit::Spades) => 17,
            (CardValue::Five, Suit::Diamonds) => 18,
            (CardValue::Five, Suit::Hearts) => 19,

            (CardValue::Six, Suit::Clubs) => 20,
            (CardValue::Six, Suit::Spades) => 21,
            (CardValue::Six, Suit::Diamonds) => 22,
            (CardValue::Six, Suit::Hearts) => 23,

            (CardValue::Seven, Suit::Clubs) => 24,
            (CardValue::Seven, Suit::Spades) => 25,
            (CardValue::Seven, Suit::Diamonds) => 26,
            (CardValue::Seven, Suit::Hearts) => 27,

            (CardValue::Eight, Suit::Clubs) => 28,
            (CardValue::Eight, Suit::Spades) => 29,
            (CardValue::Eight, Suit::Diamonds) => 30,
            (CardValue::Eight, Suit::Hearts) => 31,

            (CardValue::Nine, Suit::Clubs) => 32,
            (CardValue::Nine, Suit::Spades) => 33,
            (CardValue::Nine, Suit::Diamonds) => 34,
            (CardValue::Nine, Suit::Hearts) => 35,

            (CardValue::Ten, Suit::Clubs) => 36,
            (CardValue::Ten, Suit::Spades) => 37,
            (CardValue::Ten, Suit::Diamonds) => 38,
            (CardValue::Ten, Suit::Hearts) => 39,

            (CardValue::Jack, Suit::Clubs) => 40,
            (CardValue::Jack, Suit::Spades) => 41,
            (CardValue::Jack, Suit::Diamonds) => 42,
            (CardValue::Jack, Suit::Hearts) => 43,

            (CardValue::Queen, Suit::Clubs) => 44,
            (CardValue::Queen, Suit::Spades) => 45,
            (CardValue::Queen, Suit::Diamonds) => 46,
            (CardValue::Queen, Suit::Hearts) => 47,

            (CardValue::King, Suit::Clubs) => 48,
            (CardValue::King, Suit::Spades) => 49,
            (CardValue::King, Suit::Diamonds) => 50,
            (CardValue::King, Suit::Hearts) => 51,
        }
    }
}

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct CardAssets {
    #[asset(texture_atlas_layout(tile_size_x = 351, tile_size_y = 510, columns = 4, rows = 13))]
    card_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "cards/cards_normal.png")]
    cards: Handle<Image>
}

impl CardAssets {
    pub fn image_node(&self, card: &Card) -> ImageNode {
        let index = self.get_index(card);
        ImageNode::from_atlas_image(
            self.cards.clone(),
            TextureAtlas {
                index,
                layout: self.card_layout.clone(),
            }
        )
    }

    pub fn get_index(&self, card: &Card) -> usize {
        match (&card.value, &card.suit) {
            (CardValue::Ace, Suit::Clubs) => 0,
            (CardValue::Ace, Suit::Spades) => 1,
            (CardValue::Ace, Suit::Diamonds) => 2,
            (CardValue::Ace, Suit::Hearts) => 3,

            (CardValue::Two, Suit::Clubs) => 4,
            (CardValue::Two, Suit::Spades) => 5,
            (CardValue::Two, Suit::Diamonds) => 6,
            (CardValue::Two, Suit::Hearts) => 7,

            (CardValue::Three, Suit::Clubs) => 8,
            (CardValue::Three, Suit::Spades) => 9,
            (CardValue::Three, Suit::Diamonds) => 10,
            (CardValue::Three, Suit::Hearts) => 11,

            (CardValue::Four, Suit::Clubs) => 12,
            (CardValue::Four, Suit::Spades) => 13,
            (CardValue::Four, Suit::Diamonds) => 14,
            (CardValue::Four, Suit::Hearts) => 15,

            (CardValue::Five, Suit::Clubs) => 16,
            (CardValue::Five, Suit::Spades) => 17,
            (CardValue::Five, Suit::Diamonds) => 18,
            (CardValue::Five, Suit::Hearts) => 19,

            (CardValue::Six, Suit::Clubs) => 20,
            (CardValue::Six, Suit::Spades) => 21,
            (CardValue::Six, Suit::Diamonds) => 22,
            (CardValue::Six, Suit::Hearts) => 23,

            (CardValue::Seven, Suit::Clubs) => 24,
            (CardValue::Seven, Suit::Spades) => 25,
            (CardValue::Seven, Suit::Diamonds) => 26,
            (CardValue::Seven, Suit::Hearts) => 27,

            (CardValue::Eight, Suit::Clubs) => 28,
            (CardValue::Eight, Suit::Spades) => 29,
            (CardValue::Eight, Suit::Diamonds) => 30,
            (CardValue::Eight, Suit::Hearts) => 31,

            (CardValue::Nine, Suit::Clubs) => 32,
            (CardValue::Nine, Suit::Spades) => 33,
            (CardValue::Nine, Suit::Diamonds) => 34,
            (CardValue::Nine, Suit::Hearts) => 35,

            (CardValue::Ten, Suit::Clubs) => 36,
            (CardValue::Ten, Suit::Spades) => 37,
            (CardValue::Ten, Suit::Diamonds) => 38,
            (CardValue::Ten, Suit::Hearts) => 39,

            (CardValue::Jack, Suit::Clubs) => 40,
            (CardValue::Jack, Suit::Spades) => 41,
            (CardValue::Jack, Suit::Diamonds) => 42,
            (CardValue::Jack, Suit::Hearts) => 43,

            (CardValue::Queen, Suit::Clubs) => 44,
            (CardValue::Queen, Suit::Spades) => 45,
            (CardValue::Queen, Suit::Diamonds) => 46,
            (CardValue::Queen, Suit::Hearts) => 47,

            (CardValue::King, Suit::Clubs) => 48,
            (CardValue::King, Suit::Spades) => 49,
            (CardValue::King, Suit::Diamonds) => 50,
            (CardValue::King, Suit::Hearts) => 51,
        }
    }
}


#[derive(AssetCollection, Resource)]
pub(crate) struct Icon64Assets {
    #[asset(texture_atlas_layout(tile_size_x = 64, tile_size_y = 64, columns = 1, rows = 1))]
    icon_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "images/coin64.png")]
    icons: Handle<Image>
}

impl Icon64Assets {
    pub fn image_node(&self, index: usize) -> ImageNode {
        ImageNode::from_atlas_image(
            self.icons.clone(),
            TextureAtlas {
                index,
                layout: self.icon_layout.clone(),
            }
        )
    }

    pub const COIN: usize = 0;
}


//noinspection ALL
#[allow(non_snake_case)]
#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct CardBackAssets {
    #[asset(path = "cards/back01.png")]
    pub back: Handle<Image>,
}


// fn cards(card: ) -> impl Bundle {
//
// }
