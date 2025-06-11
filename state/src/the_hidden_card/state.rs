use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use strum::IntoEnumIterator;

use crate::cards::{Card, CardValue, Deck, Suit};
use crate::player::Player;
use crate::the_hidden_card::{Combination, HandAnalyzer};

type PlayerSetIndex = usize;
type CalleeSetIndex = usize;
type CallerIndex = usize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerSet {
    player: Option<Player>,
    hands: Vec<Card>,
    coins: i32,  // 金币
    ready: bool, // 准备状态
    score: i32,  // 分数
}

impl Default for PlayerSet {
    fn default() -> Self {
        Self {
            player: None,
            hands: Vec::with_capacity(13),
            coins: 0,

            ready: false,
            score: 0,
        }
    }
}

impl PlayerSet {
    fn add_coins(&mut self, coins: i32) {
        self.coins += coins;
    }

    fn add_score(&mut self, score: i32) {
        self.score += score;
    }

    fn has_full_of(&self, card_value: CardValue) -> bool {
        if self.hands.is_empty() {
            return false;
        }
        let count = self
            .hands
            .iter()
            .filter(|card| card.value == card_value)
            .count();
        count == 4
    }

    fn get_complement_suit(&self, card_value: CardValue) -> Vec<Card> {
        let mut suits = Vec::new();
        for suit in Suit::iter() {
            let card = Card::new(suit, card_value.clone());
            if !self.hands.contains(&card) {
                suits.push(card);
            }
        }
        suits
    }

    fn reset(&mut self) {
        self.score = 0;
        self.ready = false;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stage {
    PreGame,                  // 等带玩家入座
    DealCards,                // 发牌
    CallCard(PlayerSetIndex), // 叫牌
    InGame,                   // 游戏进行中
    Ended,                    // 游戏结束
}

pub enum GameMode {
    HiddenAllies((CallerIndex, CalleeSetIndex, Card)), // 暗叫组队
    OneVsThree(PlayerSetIndex),                        // 包牌
}

pub struct GameState {
    sets: [PlayerSet; 4],
    deck: Deck,
    hand_analyzer: HandAnalyzer,
    special_card: Card,

    mode: Option<GameMode>,
    stage: Stage,

    lead: Option<PlayerSetIndex>,               // 第一个出牌的人
    the_hidden: Option<PlayerSetIndex>,         // 隐藏队友
    current_player_set: Option<PlayerSetIndex>, // 当前出牌
    is_hidden_card_shown: bool,

    last_played_set_index: Option<PlayerSetIndex>,
    last_played_cards: Option<Combination>,
    table_score_counter: i32,
}

impl GameState {
    fn nwe() -> Self {
        let mut sets: [PlayerSet; 4] = Default::default();

        Self {
            sets,
            deck: Deck::new(),
            hand_analyzer: HandAnalyzer::new(),
            special_card: Card::new(Suit::Spades, CardValue::Seven), //  特殊牌黑桃7喊牌

            mode: None,
            stage: Stage::PreGame,

            lead: None,
            the_hidden: None,
            current_player_set: None,
            last_played_cards: None,
            last_played_set_index: None,
            is_hidden_card_shown: false,
            table_score_counter: 0,
        }
    }

    fn get_sets(&self) -> &[PlayerSet; 4] {
        &self.sets
    }

    fn get_active_set(&self) -> Option<PlayerSet> {
        if let Some(index) = self.current_player_set {
            Some(self.sets[index].clone())
        } else {
            None
        }
    }

    fn next_player(&mut self) {
        self.current_player_set = match self.current_player_set {
            Some(current) => Some((current + 1) % 4), // 循环递增
            None => Some(0), // 如果当前无玩家，从0开始, 正常情况下不会匹配到这里
        };
    }

    /// 获取可叫的牌，叫牌阶段
    fn get_callable_cards(&self) -> Option<Vec<Card>> {
        if let Stage::CallCard(caller_index) = self.stage {
            let set = &self.sets[caller_index];
            if !set.has_full_of(CardValue::Two) {
                return Some(set.get_complement_suit(CardValue::Two));
            } else if !set.has_full_of(CardValue::Ace) {
                return Some(set.get_complement_suit(CardValue::Ace));
            } else if !set.has_full_of(CardValue::King) {
                return Some(set.get_complement_suit(CardValue::King));
            } else if !set.has_full_of(CardValue::Queen) {
                return Some(set.get_complement_suit(CardValue::Queen));
            }
        }
        None
    }

    /// 重置玩家席状态和游戏状态
    fn reset(&mut self) {
        self.sets.iter_mut().for_each(|set| set.reset());
        self.lead = None;
        self.the_hidden = None;
        self.current_player_set = None;
        self.last_played_cards = None;
        self.last_played_set_index = None;
        self.is_hidden_card_shown = false;
    }

    /// 洗牌
    fn shuffle(&mut self) {
        self.deck.shuffle();
    }

    /// 发牌
    fn deal(&mut self) {
        let mut deck = VecDeque::from(self.deck.get().clone());
        for set in self.sets.iter_mut() {
            for _ in 0..13 {
                set.hands.push(deck.pop_front().unwrap())
            }
        }
    }

    /// 第一步：准备游戏，Stage::[PreGame, EndGame] 状态下可执行，执行后游戏进入发牌状态 Stage::DealCards
    /// 当所有玩家准备好后开始游戏
    /// 一轮游戏结束后需要重置玩家准备状态
    /// 初始化游戏状态
    /// 1，重置状态
    /// 2，洗牌
    /// 3，发牌
    fn prepare_game(&mut self) {
        self.reset();
        self.shuffle();
        self.deal();
        self.stage = Stage::DealCards;
    }

    /// 第二步：当前状态：Stage::DealCards 执行后进入下一个状态：Stage::CallCard(caller_index)
    /// 由游戏系统发牌动作完成后执行
    /// 改状态下持有特殊牌的玩家有两个按钮： 叫牌（最多4个牌型按钮）和一个包牌按钮
    /// 其他玩家只有一个按钮： 包牌
    /// 按照优先原则
    fn to_call_card_stage(&mut self) {
        // 找出有黑桃7的玩家的索引
        let caller_index = self
            .sets
            .iter()
            .position(|set| set.hands.iter().any(|card| *card == self.special_card));

        self.stage = Stage::CallCard(caller_index.unwrap());
    }

    /// 第三步：当前状态：Stage::CallCard(caller_index) 执行后进入下一个状态：Stage::InGame
    /// 设置游戏模式为 GameMode::OneVsThree(player_set_index)
    /// 有玩家包牌后始游戏
    fn blocking_start(&mut self, player_set_index: PlayerSetIndex) {
        if !matches!(self.stage, Stage::CallCard(_)) {
            return;
        }
        self.mode = Some(GameMode::OneVsThree(player_set_index));
        self.current_player_set = Some(player_set_index);

        // self.first = Some(player_set_index);
        self.stage = Stage::InGame;
    }

    /// 第三步：当前状态：Stage::CallCard(caller_index) 执行后进入下一个状态：Stage::InGame
    /// 设置游戏模式为 GameMode::HiddenAllies((caller_index, callee_index, call_card))
    /// 玩家叫牌后开始
    fn call_card_start(&mut self, caller_index: CallerIndex, call_card: Card) {
        if !matches!(self.stage, Stage::CallCard(_)) {
            return;
        }

        let callee_index = self
            .sets
            .iter()
            .position(|set| set.hands.iter().any(|card| *card == call_card));

        if let Some(callee_index) = callee_index {
            self.mode = Some(GameMode::HiddenAllies((
                caller_index,
                callee_index,
                call_card,
            )));
            self.stage = Stage::InGame;
            self.current_player_set = Some(caller_index);
        } else {
            self.mode = None;
            self.stage = Stage::Ended;
        }
    }

    fn play_card(
        &mut self,
        player_set_index: PlayerSetIndex,
        cards: Vec<Card>,
    ) -> Result<(), String> {
        let combo = self.hand_analyzer.set(cards).analyze();

        if combo == Combination::Invalid {
            return Err("无效牌型".to_string());
        }
        if let Some(ref last_combo) = self.last_played_cards {
            if !combo.gt(last_combo) {
                return Err("牌型太弱".to_string());
            }
        }

        self.last_played_set_index = Some(player_set_index);
        self.add_combo_to_table_score(&combo);
        self.last_played_cards = Some(combo);
        self.next_player();
        Ok(())
    }

    fn pass(&mut self) {
        self.next_player();
        if let (Some(current), Some(last_played)) =
            (self.current_player_set, self.last_played_set_index)
        {
            if current == last_played {
                let player_set = self.sets.get_mut(current).unwrap();
                // 将桌面分数加到坐席分数中
                player_set.add_score(self.table_score_counter);

                // 将桌面分数清零
                self.table_score_counter = 0;
                self.last_played_set_index = None;
                self.last_played_cards = None;
            }
        }
    }

    fn add_combo_to_table_score(&mut self, combo: &Combination) {
        match combo {
            Combination::Single(_) => {
                self.table_score_counter += 1;
            },
            Combination::Pair(_) => {
                self.table_score_counter += 2;
            },
            Combination::Straight(cards) => {
                self.table_score_counter += cards.len() as i32;
            },
            Combination::ThreeOfAKind(_) => {
                self.table_score_counter += 3;
            },
            Combination::ThreeStraitPair(_) => {
                self.table_score_counter += 6;
            },
            Combination::FourOfAKind(_) => {
                self.table_score_counter += 4;
            },
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::{Card, CardValue::*, Suit::*};
    use rand::rng;
    use rand::seq::IndexedRandom;

    // 创建测试用的特殊牌 (黑桃7)
    fn special_card() -> Card {
        Card::new(Suit::Spades, CardValue::Seven)
    }

    #[test]
    fn test_initial_state() {
        let state = GameState::nwe();
        assert_eq!(state.stage, Stage::PreGame);
        assert!(state.mode.is_none());
        assert!(state.lead.is_none());
        assert!(state.the_hidden.is_none());
        assert!(state.current_player_set.is_none());
    }

    #[test]
    fn test_pre_start_flow() {
        let mut state = GameState::nwe();

        // 准备阶段
        state.prepare_game();
        assert_eq!(state.stage, Stage::DealCards);

        // 检查发牌情况
        for set in &state.sets {
            assert_eq!(set.hands.len(), 13);
        }

        // 验证特殊牌存在
        let has_special = state
            .sets
            .iter()
            .any(|set| set.hands.contains(&special_card()));
        assert!(has_special);
    }

    #[test]
    fn test_call_card_stage_transition() {
        let mut state = GameState::nwe();
        state.shuffle();
        state.prepare_game();

        state.to_call_card_stage();
        match state.stage {
            Stage::CallCard(caller_index) => {
                // 验证叫牌者持有特殊牌
                assert!(state.sets[caller_index].hands.contains(&special_card()));
            },
            _ => panic!("Should be in CallCard stage"),
        }
    }

    #[test]
    fn test_blocking_start() {
        let mut state = GameState::nwe();
        state.prepare_game();
        state.to_call_card_stage();

        let blocker_index = 2;
        state.blocking_start(blocker_index);

        // 验证游戏模式
        match state.mode {
            Some(GameMode::OneVsThree(index)) => {
                assert_eq!(index, blocker_index);
            },
            _ => panic!("Wrong game mode"),
        }

        assert_eq!(state.stage, Stage::InGame);
        assert_eq!(state.current_player_set, Some(blocker_index));
    }

    #[test]
    fn test_call_card_start() {
        let mut state = GameState::nwe();
        state.prepare_game();
        state.to_call_card_stage();

        let callable = state.get_callable_cards().unwrap();
        let call_card = callable.choose(&mut rng()).unwrap(); // 任意测试牌

        let Stage::CallCard(caller_index) = state.stage else {
            panic!("Not in Stage::CallCard")
        };

        state.call_card_start(caller_index, call_card.clone());

        // 验证游戏模式
        match state.mode {
            Some(GameMode::HiddenAllies((caller, callee, card))) => {
                assert_eq!(caller, caller_index);
                assert_eq!(card, call_card.clone());
            },
            _ => panic!("Wrong game mode"),
        }
        assert_eq!(state.stage, Stage::InGame);
    }

    #[test]
    fn test_play_card_validation() {
        let mut state = GameState::nwe();
        state.stage = Stage::InGame; // 设置为游戏中

        // 构造合法牌型
        let valid_cards = vec![
            Card::new(Suit::Hearts, CardValue::Ten),
            Card::new(Suit::Hearts, CardValue::Ten),
        ];

        // 构造非法牌型
        let invalid_cards = vec![
            Card::new(Suit::Hearts, CardValue::Ten),
            Card::new(Suit::Spades, CardValue::Jack),
        ];

        // 验证出牌
        assert!(state.play_card(0, valid_cards).is_ok());
        assert!(state.play_card(0, invalid_cards).is_err());
    }



    #[test]
    fn test_player_set_operations() {
        let mut set = PlayerSet::default();

        // 金币操作
        set.add_coins(5);
        assert_eq!(set.coins, 5);

        // 分数操作
        set.add_score(10);
        assert_eq!(set.score, 10);

        // 重置状态
        set.reset();
        assert_eq!(set.score, 0);
        assert!(!set.ready);
        // 注意：金币和手牌不会被重置
        assert_eq!(set.coins, 5);
    }
}
