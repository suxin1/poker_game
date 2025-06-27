use crate::prelude::*;
use shared::cards::{Card, CardValue, Suit};

pub(super) fn plugin(app: &mut App) {
    app.init_collection::<CardAssets>();
    app.init_collection::<IndicatorAsset>();
}

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct IndicatorAsset {
    #[asset(path = "sprite/arrow-sheet.png")]
    pub arrow: Handle<Image>,
}


//noinspection ALL
#[allow(non_snake_case)]
#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct CardAssets {
    #[asset(path = "cards/2C.png")]
    pub C2: Handle<Image>,
    #[asset(path = "cards/2D.png")]
    pub D2: Handle<Image>,
    #[asset(path = "cards/2H.png")]
    pub H2: Handle<Image>,
    #[asset(path = "cards/2S.png")]
    pub S2: Handle<Image>,
    #[asset(path = "cards/3C.png")]
    pub C3: Handle<Image>,
    #[asset(path = "cards/3D.png")]
    pub D3: Handle<Image>,
    #[asset(path = "cards/3H.png")]
    pub H3: Handle<Image>,
    #[asset(path = "cards/3S.png")]
    pub S3: Handle<Image>,
    #[asset(path = "cards/4C.png")]
    pub C4: Handle<Image>,
    #[asset(path = "cards/4D.png")]
    pub D4: Handle<Image>,
    #[asset(path = "cards/4H.png")]
    pub H4: Handle<Image>,
    #[asset(path = "cards/4S.png")]
    pub S4: Handle<Image>,
    #[asset(path = "cards/5C.png")]
    pub C5: Handle<Image>,
    #[asset(path = "cards/5D.png")]
    pub D5: Handle<Image>,
    #[asset(path = "cards/5H.png")]
    pub H5: Handle<Image>,
    #[asset(path = "cards/5S.png")]
    pub S5: Handle<Image>,
    #[asset(path = "cards/6C.png")]
    pub C6: Handle<Image>,
    #[asset(path = "cards/6D.png")]
    pub D6: Handle<Image>,
    #[asset(path = "cards/6H.png")]
    pub H6: Handle<Image>,
    #[asset(path = "cards/6S.png")]
    pub S6: Handle<Image>,
    #[asset(path = "cards/7C.png")]
    pub C7: Handle<Image>,
    #[asset(path = "cards/7D.png")]
    pub D7: Handle<Image>,
    #[asset(path = "cards/7H.png")]
    pub H7: Handle<Image>,
    #[asset(path = "cards/7S.png")]
    pub S7: Handle<Image>,
    #[asset(path = "cards/8C.png")]
    pub C8: Handle<Image>,
    #[asset(path = "cards/8D.png")]
    pub D8: Handle<Image>,
    #[asset(path = "cards/8H.png")]
    pub H8: Handle<Image>,
    #[asset(path = "cards/8S.png")]
    pub S8: Handle<Image>,
    #[asset(path = "cards/9C.png")]
    pub C9: Handle<Image>,
    #[asset(path = "cards/9D.png")]
    pub D9: Handle<Image>,
    #[asset(path = "cards/9H.png")]
    pub H9: Handle<Image>,
    #[asset(path = "cards/9S.png")]
    pub S9: Handle<Image>,
    #[asset(path = "cards/10C.png")]
    pub C10: Handle<Image>,
    #[asset(path = "cards/10D.png")]
    pub D10: Handle<Image>,
    #[asset(path = "cards/10H.png")]
    pub H10: Handle<Image>,
    #[asset(path = "cards/10S.png")]
    pub S10: Handle<Image>,
    #[asset(path = "cards/JC.png")]
    pub CJ: Handle<Image>,
    #[asset(path = "cards/JD.png")]
    pub DJ: Handle<Image>,
    #[asset(path = "cards/JH.png")]
    pub HJ: Handle<Image>,
    #[asset(path = "cards/JS.png")]
    pub SJ: Handle<Image>,
    #[asset(path = "cards/QC.png")]
    pub CQ: Handle<Image>,
    #[asset(path = "cards/QD.png")]
    pub DQ: Handle<Image>,
    #[asset(path = "cards/QH.png")]
    pub HQ: Handle<Image>,
    #[asset(path = "cards/QS.png")]
    pub SQ: Handle<Image>,
    #[asset(path = "cards/KC.png")]
    pub CK: Handle<Image>,
    #[asset(path = "cards/KD.png")]
    pub DK: Handle<Image>,
    #[asset(path = "cards/KH.png")]
    pub HK: Handle<Image>,
    #[asset(path = "cards/KS.png")]
    pub SK: Handle<Image>,
    #[asset(path = "cards/AC.png")]
    pub CA: Handle<Image>,
    #[asset(path = "cards/AD.png")]
    pub DA: Handle<Image>,
    #[asset(path = "cards/AH.png")]
    pub HA: Handle<Image>,
    #[asset(path = "cards/AS.png")]
    pub SA: Handle<Image>,
    #[asset(path = "cards/back01.png")]
    pub back: Handle<Image>,
}

impl CardAssets {
    pub fn get_card_img(&self, card: &Card) -> Handle<Image> {
        match (&card.value, &card.suit) {
            (CardValue::Ace, Suit::Spades) => self.SA.clone(),
            (CardValue::Ace, Suit::Hearts) => self.HA.clone(),
            (CardValue::Ace, Suit::Diamonds) => self.DA.clone(),
            (CardValue::Ace, Suit::Clubs) => self.CA.clone(),

            (CardValue::Two, Suit::Spades) => self.S2.clone(),
            (CardValue::Two, Suit::Hearts) => self.H2.clone(),
            (CardValue::Two, Suit::Diamonds) => self.D2.clone(),
            (CardValue::Two, Suit::Clubs) => self.C2.clone(),

            (CardValue::Three, Suit::Spades) => self.S3.clone(),
            (CardValue::Three, Suit::Hearts) => self.H3.clone(),
            (CardValue::Three, Suit::Diamonds) => self.D3.clone(),
            (CardValue::Three, Suit::Clubs) => self.C3.clone(),

            (CardValue::Four, Suit::Spades) => self.S4.clone(),
            (CardValue::Four, Suit::Hearts) => self.H4.clone(),
            (CardValue::Four, Suit::Diamonds) => self.D4.clone(),
            (CardValue::Four, Suit::Clubs) => self.C4.clone(),

            (CardValue::Five, Suit::Spades) => self.S5.clone(),
            (CardValue::Five, Suit::Hearts) => self.H5.clone(),
            (CardValue::Five, Suit::Diamonds) => self.D5.clone(),
            (CardValue::Five, Suit::Clubs) => self.C5.clone(),

            (CardValue::Six, Suit::Spades) => self.S6.clone(),
            (CardValue::Six, Suit::Hearts) => self.H6.clone(),
            (CardValue::Six, Suit::Diamonds) => self.D6.clone(),
            (CardValue::Six, Suit::Clubs) => self.C6.clone(),

            (CardValue::Seven, Suit::Spades) => self.S7.clone(),
            (CardValue::Seven, Suit::Hearts) => self.H7.clone(),
            (CardValue::Seven, Suit::Diamonds) => self.D7.clone(),
            (CardValue::Seven, Suit::Clubs) => self.C7.clone(),

            (CardValue::Eight, Suit::Spades) => self.S8.clone(),
            (CardValue::Eight, Suit::Hearts) => self.H8.clone(),
            (CardValue::Eight, Suit::Diamonds) => self.D8.clone(),
            (CardValue::Eight, Suit::Clubs) => self.C8.clone(),

            (CardValue::Nine, Suit::Spades) => self.S9.clone(),
            (CardValue::Nine, Suit::Hearts) => self.H9.clone(),
            (CardValue::Nine, Suit::Diamonds) => self.D9.clone(),
            (CardValue::Nine, Suit::Clubs) => self.C9.clone(),

            (CardValue::Ten, Suit::Spades) => self.S10.clone(),
            (CardValue::Ten, Suit::Hearts) => self.H10.clone(),
            (CardValue::Ten, Suit::Diamonds) => self.D10.clone(),
            (CardValue::Ten, Suit::Clubs) => self.C10.clone(),

            (CardValue::Jack, Suit::Spades) => self.SJ.clone(),
            (CardValue::Jack, Suit::Hearts) => self.HJ.clone(),
            (CardValue::Jack, Suit::Diamonds) => self.DJ.clone(),
            (CardValue::Jack, Suit::Clubs) => self.CJ.clone(),

            (CardValue::Queen, Suit::Spades) => self.SQ.clone(),
            (CardValue::Queen, Suit::Hearts) => self.HQ.clone(),
            (CardValue::Queen, Suit::Diamonds) => self.DQ.clone(),
            (CardValue::Queen, Suit::Clubs) => self.CQ.clone(),

            (CardValue::King, Suit::Spades) => self.SK.clone(),
            (CardValue::King, Suit::Hearts) => self.HK.clone(),
            (CardValue::King, Suit::Diamonds) => self.DK.clone(),
            (CardValue::King, Suit::Clubs) => self.CK.clone(),
        }
    }
}

// fn cards(card: ) -> impl Bundle {
//
// }
