use std::cmp::Ordering;
use std::convert::TryInto;

use crate::cards::{Card, CardNumericValue, CardValue};

impl CardNumericValue for CardValue {
    fn int(&self) -> u8 {
        match self {
            CardValue::Three => 1,
            CardValue::Four => 2,
            CardValue::Five => 3,
            CardValue::Six => 4,
            CardValue::Seven => 5,
            CardValue::Eight => 6,
            CardValue::Nine => 7,
            CardValue::Ten => 8,
            CardValue::Jack => 9,
            CardValue::Queen => 10,
            CardValue::King => 11,
            CardValue::Ace => 12,
            CardValue::Two => 13,
        }
    }
}

/// 牌型枚举
#[derive(Eq, PartialEq, PartialOrd, Ord)]
pub enum Combination {
    Single(Card),               // 单张
    Pair([Card; 2]),            // 对子
    Straight(Vec<Card>),        // 顺子 （3张起）
    ThreeOfAKind([Card; 3]),    // 三张炸弹
    ThreeStraitPair([Card; 6]), // 板板炮 3连对
    FourOfAKind([Card; 4]),     // 四张炸弹
    Invalid,                    // 无效牌型
}

impl Combination {
    pub fn is_boom(&self) -> bool {
        matches!(
            self,
            Combination::ThreeOfAKind(_)
                | Combination::FourOfAKind(_)
                | Combination::ThreeStraitPair(_)
        )
    }

    pub fn gt(&self, last_combo: &Self) -> bool {
        match (self, &last_combo) {
            (Self::Single(a), Self::Single(b)) => a > b,
            (Self::Pair(a), Self::Pair(b)) => a > b,
            (Self::Straight(a), Self::Straight(b)) => {
                if a.len() != b.len() {
                    return false;
                }
                let mut a = a.clone();
                let mut b = b.clone();
                a.sort();
                b.sort();
                a > b
            },
            (Self::ThreeStraitPair(a), Self::ThreeStraitPair(b)) => {
                let mut a = a.clone();
                let mut b = b.clone();
                a.sort();
                b.sort();
                a > b
            },
            _ => {
                if self.is_boom() {
                    return self > last_combo;
                }
                false
            },
        }
    }

    pub fn analyze(cards: Vec<Card>) {
        let analyzer = HandAnalyzer::from_cards(cards);
        analyzer.analyze();
    }
}

/// 牌型分析
pub struct HandAnalyzer(Vec<Card>);

impl HandAnalyzer {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_cards(cards: Vec<Card>) -> Self {
        Self(cards)
    }

    pub fn set(&mut self, mut cards: Vec<Card>) -> &mut Self {
        cards.sort_by(|a, b| a.value.int().cmp(&b.value.int()));
        self.0 = cards;
        self
    }

    pub fn analyze(&self) -> Combination {
        match self.0.len() {
            0 => Combination::Invalid,
            1 => Combination::Single(self.0[0].clone()),
            2 => {
                if self.is_pair() {
                    Combination::Pair([self.0[0].clone(), self.0[1].clone()])
                } else {
                    Combination::Invalid
                }
            },
            3 => {
                if self.is_consecutive() {
                    Combination::Straight(self.0.clone())
                } else if self.is_three_of_a_kind() {
                    Combination::ThreeOfAKind([
                        self.0[0].clone(),
                        self.0[1].clone(),
                        self.0[2].clone(),
                    ])
                } else {
                    Combination::Invalid
                }
            },
            4 => {
                if self.is_four_of_a_kind() {
                    Combination::FourOfAKind([
                        self.0[0].clone(),
                        self.0[1].clone(),
                        self.0[2].clone(),
                        self.0[3].clone(),
                    ])
                } else if self.is_consecutive() {
                    Combination::Straight(self.0.clone())
                } else {
                    Combination::Invalid
                }
            },
            6 => {
                if self.is_three_strait_pair() {
                    Combination::ThreeStraitPair(
                        self.0.clone().try_into().expect("Expected 6 cards"),
                    )
                } else if self.is_consecutive() {
                    Combination::Straight(self.0.clone())
                } else {
                    Combination::Invalid
                }
            },
            _ => {
                if self.0.len() >= 3 && self.is_consecutive() {
                    Combination::Straight(self.0.clone())
                } else {
                    Combination::Invalid
                }
            },
        }
    }

    fn is_pair(&self) -> bool {
        self.0.len() == 2 && self.0[0].value == self.0[1].value
    }

    fn is_consecutive(&self) -> bool {
        let len = self.0.len();
        if len < 3 {
            return false;
        }

        // 最大牌不能大过 A
        if self.0.last().unwrap().value.int() > CardValue::Ace.int() {
            return false;
        }

        // 检查连续递增
        for i in 0..len - 1 {
            if self.0[i].value.int() + 1 != self.0[i + 1].value.int() {
                return false;
            }
        }
        true
    }

    fn is_three_of_a_kind(&self) -> bool {
        self.0.len() == 3
            && self.0[0].value == self.0[1].value
            && self.0[1].value == self.0[2].value
    }

    fn is_four_of_a_kind(&self) -> bool {
        self.0.len() == 4
            && self.0[0].value == self.0[1].value
            && self.0[1].value == self.0[2].value
            && self.0[2].value == self.0[3].value
    }

    fn is_three_strait_pair(&self) -> bool {
        if self.0.len() != 6 {
            return false;
        }
        // 最大牌不能大过 A
        if self.0.last().unwrap().value.int() > CardValue::Ace.int() {
            return false;
        }

        let mut values = Vec::new();
        for chunk in self.0.chunks_exact(2) {
            if chunk[0].value != chunk[1].value {
                return false;
            }
            values.push(chunk[0].value.int());
        }

        values[0] + 1 == values[1] && values[1] + 1 == values[2]
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use super::*;
    use crate::cards::CardValue::{Ace, Five, Four, Jack, King, Nine, Queen, Six, Ten, Three, Two};
    use crate::cards::Suit::Spades;
    use crate::cards::{Card, CardValue, Suit};

    // 辅助函数：创建单张牌
    fn card(value: CardValue, suit: Suit) -> Card {
        Card { value, suit }
    }

    // 测试单张牌型
    #[test]
    fn test_single_card() {
        let mut analyzer = HandAnalyzer::new();
        let cards = vec![card(CardValue::Ace, Suit::Hearts)];
        analyzer.set(cards);
        match analyzer.analyze() {
            Combination::Single(c) => assert_eq!(c.value, CardValue::Ace),
            _ => panic!("Expected Single"),
        }
    }

    // 测试对子牌型
    #[test]
    fn test_pair() {
        let mut analyzer = HandAnalyzer::new();
        let cards = vec![
            card(CardValue::Ten, Suit::Hearts),
            card(CardValue::Ten, Suit::Diamonds),
        ];
        analyzer.set(cards);
        match analyzer.analyze() {
            Combination::Pair(pair) => {
                assert_eq!(pair[0].value, CardValue::Ten);
                assert_eq!(pair[1].value, CardValue::Ten);
            },
            _ => panic!("Expected Pair"),
        }
    }

    // 测试三张炸弹
    #[test]
    fn test_three_of_a_kind() {
        let mut analyzer = HandAnalyzer::new();
        let cards = vec![
            card(CardValue::Seven, Suit::Hearts),
            card(CardValue::Seven, Suit::Diamonds),
            card(CardValue::Seven, Suit::Clubs),
        ];
        analyzer.set(cards);
        match analyzer.analyze() {
            Combination::ThreeOfAKind(triple) => {
                assert_eq!(triple[0].value, CardValue::Seven);
                assert_eq!(triple[1].value, CardValue::Seven);
                assert_eq!(triple[2].value, CardValue::Seven);
            },
            _ => panic!("Expected ThreeOfAKind"),
        }
    }

    // 测试四张炸弹
    #[test]
    fn test_four_of_a_kind() {
        let mut analyzer = HandAnalyzer::new();
        let cards = vec![
            card(CardValue::King, Suit::Hearts),
            card(CardValue::King, Suit::Diamonds),
            card(CardValue::King, Suit::Clubs),
            card(CardValue::King, Suit::Spades),
        ];
        analyzer.set(cards);
        match analyzer.analyze() {
            Combination::FourOfAKind(quad) => {
                assert_eq!(quad[0].value, CardValue::King);
                assert_eq!(quad[3].value, CardValue::King);
            },
            _ => panic!("Expected FourOfAKind"),
        }
    }

    // 测试板板炮（三连对）
    #[test]
    fn test_three_strait_pair() {
        let mut analyzer = HandAnalyzer::new();
        let cards = vec![
            card(CardValue::Five, Suit::Hearts),
            card(CardValue::Five, Suit::Diamonds),
            card(CardValue::Six, Suit::Hearts),
            card(CardValue::Six, Suit::Diamonds),
            card(CardValue::Seven, Suit::Hearts),
            card(CardValue::Seven, Suit::Diamonds),
        ];
        analyzer.set(cards);
        match analyzer.analyze() {
            Combination::ThreeStraitPair(six_cards) => {
                assert_eq!(six_cards[0].value, CardValue::Five);
                assert_eq!(six_cards[1].value, CardValue::Five);
                assert_eq!(six_cards[4].value, CardValue::Seven);
            },
            _ => panic!("Expected ThreeStraitPair"),
        }
    }

    // 测试顺子（3张）
    #[test]
    fn test_short_straight() {
        let mut analyzer = HandAnalyzer::new();
        let cards = vec![
            card(CardValue::Three, Suit::Hearts),
            card(CardValue::Four, Suit::Diamonds),
            card(CardValue::Five, Suit::Clubs),
        ];
        analyzer.set(cards);
        match analyzer.analyze() {
            Combination::Straight(straight) => {
                assert_eq!(straight.len(), 3);
                assert_eq!(straight[0].value, CardValue::Three);
                assert_eq!(straight[2].value, CardValue::Five);
            },
            _ => panic!("Expected Straight"),
        }
    }

    // 测试顺子（5张）
    #[test]
    fn test_long_straight() {
        let mut analyzer = HandAnalyzer::new();
        let cards = vec![
            card(CardValue::Nine, Suit::Hearts),
            card(CardValue::Ten, Suit::Diamonds),
            card(CardValue::Jack, Suit::Clubs),
            card(CardValue::Queen, Suit::Spades),
            card(CardValue::King, Suit::Hearts),
        ];
        analyzer.set(cards);
        match analyzer.analyze() {
            Combination::Straight(straight) => {
                assert_eq!(straight.len(), 5);
                assert_eq!(straight[0].value, CardValue::Nine);
                assert_eq!(straight[4].value, CardValue::King);
            },
            _ => panic!("Expected Straight"),
        }
    }

    // 测试无效牌型：两张不同牌
    #[test]
    fn test_invalid_pair() {
        let mut analyzer = HandAnalyzer::new();
        let cards = vec![
            card(CardValue::Ace, Suit::Hearts),
            card(CardValue::King, Suit::Diamonds),
        ];
        analyzer.set(cards);
        assert!(matches!(analyzer.analyze(), Combination::Invalid));
    }

    // 测试无效牌型：包含2的顺子
    #[test]
    fn test_straight_with_two() {
        let mut analyzer = HandAnalyzer::new();
        let cards = vec![
            card(CardValue::Ace, Suit::Hearts),
            card(CardValue::Two, Suit::Diamonds),
            card(CardValue::Three, Suit::Clubs),
        ];
        analyzer.set(cards);
        assert!(matches!(analyzer.analyze(), Combination::Invalid));
    }

    // 测试无效牌型：不连续的牌
    #[test]
    fn test_non_consecutive() {
        let mut analyzer = HandAnalyzer::new();
        let cards = vec![
            card(CardValue::Five, Suit::Hearts),
            card(CardValue::Seven, Suit::Diamonds),
            card(CardValue::Eight, Suit::Clubs),
        ];
        analyzer.set(cards);
        assert!(matches!(analyzer.analyze(), Combination::Invalid));
    }

    // 测试无效牌型：错误的板板炮（对子不连续）
    #[test]
    fn test_invalid_three_strait_pair() {
        let mut analyzer = HandAnalyzer::new();
        let cards = vec![
            card(CardValue::Five, Suit::Hearts),
            card(CardValue::Five, Suit::Diamonds),
            card(CardValue::Six, Suit::Hearts),
            card(CardValue::Six, Suit::Diamonds),
            card(CardValue::Eight, Suit::Hearts), // 这里应该是7
            card(CardValue::Eight, Suit::Diamonds),
        ];
        analyzer.set(cards);
        assert!(matches!(analyzer.analyze(), Combination::Invalid));
    }

    // 测试边界情况：空牌
    #[test]
    fn test_empty_hand() {
        let analyzer = HandAnalyzer::new();
        assert!(matches!(analyzer.analyze(), Combination::Invalid));
    }

    // 测试特殊顺子：A-K-Q-J-10
    #[test]
    fn test_high_straight() {
        let mut analyzer = HandAnalyzer::new();
        let cards = vec![
            card(CardValue::Ten, Suit::Hearts),
            card(CardValue::Jack, Suit::Diamonds),
            card(CardValue::Queen, Suit::Clubs),
            card(CardValue::King, Suit::Spades),
            card(CardValue::Ace, Suit::Hearts),
        ];
        analyzer.set(cards);
        match analyzer.analyze() {
            Combination::Straight(straight) => {
                assert_eq!(straight[0].value, CardValue::Ten);
                assert_eq!(straight[4].value, CardValue::Ace);
            },
            _ => panic!("Expected Straight"),
        }
    }

    #[test]
    fn test_card_comparison() {
        use Combination::*;
        let card_10_s = card(CardValue::Ten, Suit::Spades);
        let card_10_a = card(CardValue::Ten, Suit::Hearts);
        let card_10_d = card(CardValue::Ten, Suit::Diamonds);
        let card_10_c = card(CardValue::Ten, Suit::Clubs);

        assert!(!Pair([card_10_c, card_10_d]).gt(&Pair([card_10_s, card_10_a])));
        // 单张比较
        assert!(&Single(Card::new(Spades, Ace)).gt(&Single(Card::new(Spades, King))));

        // 对子比较
        assert!(
            !Pair([Card::new(Spades, Queen), Card::new(Spades, Queen)])
                .gt(&Pair([Card::new(Spades, King), Card::new(Spades, King)]))
        );

        // 顺子比较
        let straight_3_start_with_10 = Straight(vec![
            Card::new(Spades, Ten),
            Card::new(Spades, Jack),
            Card::new(Spades, Queen),
        ]);
        let straight_3_start_with_9 = Straight(vec![
            Card::new(Spades, Nine),
            Card::new(Spades, Ten),
            Card::new(Spades, Jack),
        ]);
        let straight_3_start_with_9_nord = Straight(vec![
            Card::new(Spades, Jack),
            Card::new(Spades, Ten),
            Card::new(Spades, Nine),
        ]);
        let straight_4_start_with_9 = Straight(vec![
            Card::new(Spades, Nine),
            Card::new(Spades, Ten),
            Card::new(Spades, Jack),
            Card::new(Spades, Ace),
        ]);

        let three_boom_2 = ThreeOfAKind([
            Card::new(Spades, Two),
            Card::new(Spades, Two),
            Card::new(Spades, Two),
        ]);

        let four_boom_3 = FourOfAKind([
            Card::new(Spades, Three),
            Card::new(Spades, Three),
            Card::new(Spades, Three),
            Card::new(Spades, Three),
        ]);

        let four_boom_2 = FourOfAKind([
            Card::new(Spades, Two),
            Card::new(Spades, Two),
            Card::new(Spades, Two),
            Card::new(Spades, Two),
        ]);

        let strait_pair_start_with_3 = ThreeStraitPair([
            Card::new(Spades, Five),
            Card::new(Spades, Five),
            Card::new(Spades, Three),
            Card::new(Spades, Three),
            Card::new(Spades, Four),
            Card::new(Spades, Four),
        ]);

        let strait_pair_start_with_4 = ThreeStraitPair([
            Card::new(Spades, Four),
            Card::new(Spades, Four),
            Card::new(Spades, Five),
            Card::new(Spades, Five),
            Card::new(Spades, Six),
            Card::new(Spades, Six),
        ]);

        let boom_4_a = FourOfAKind([
            Card::new(Spades, Four),
            Card::new(Spades, Four),
            Card::new(Spades, Four),
            Card::new(Spades, Four),
        ]);
        //  顺子比较
        assert!(straight_3_start_with_10.gt(&straight_3_start_with_9));
        assert!(!straight_3_start_with_9.gt(&straight_3_start_with_10));
        //  无序顺子比较
        assert!(straight_3_start_with_10.gt(&straight_3_start_with_9_nord));
        // 不同长度顺子比较
        assert!(!straight_4_start_with_9.gt(&straight_3_start_with_9));
        // 炸弹比较
        assert!(!four_boom_3.gt(&four_boom_3));
        assert!(!four_boom_3.gt(&four_boom_2));
        assert!(!three_boom_2.gt(&four_boom_3));
        assert!(strait_pair_start_with_4.gt(&strait_pair_start_with_3));

        // 炸弹比较
        assert!(
            boom_4_a
            .gt(&straight_3_start_with_9)
        );
    }
}
