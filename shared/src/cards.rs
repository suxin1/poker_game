use std::cmp::Ordering;
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Serialize, Deserialize, EnumIter)]
pub enum Suit {
    Spades,   // 黑桃
    Hearts,   // 红桃
    Diamonds, // 方片
    Clubs,    // 梅花
}

/// Card 手动实现了 Ord trait 来实现只针对CardValue来排序，忽略suit，仅用于排序
/// 注意，为确保每张牌（Card）的唯一性，不要轻易手动实现 PartialEq， Eq。
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Card {
    pub value: CardValue,
    pub suit: Suit,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other)) // 复用 Ord 的实现
    }
}
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl Card {
    pub fn new(suit: Suit, value: CardValue) -> Self {
        Self { suit, value }
    }
}

#[derive(Debug, Clone)]
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