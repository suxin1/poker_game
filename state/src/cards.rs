use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use rand::rng;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, PartialOrd, Ord)]
pub enum CardValue {
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Two,
}

pub trait CardNumericValue {
    fn int(&self) -> u8;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, EnumIter)]
pub enum Suit {
    Spades,   // 黑桃
    Hearts,   // 红桃
    Diamonds, // 方片
    Clubs,    // 梅花
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct Card {
    pub value: CardValue,
    pub suit: Suit,
}

impl Card {
    pub fn new(suit: Suit, value: CardValue) -> Self {
        Self { suit, value }
    }
}

pub struct Deck {
    value: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut deck = Vec::with_capacity(52);

        for suit in Suit::iter() {
            for value in CardValue::iter() {
                deck.push(Card::new(suit.clone(), value.clone()))
            }
        }
        Self { value: deck }
    }

    pub fn shuffle(&mut self) {
        self.value.shuffle(&mut rng());
    }

    pub fn get(&self) -> &Vec<Card> {
        &self.value
    }
}