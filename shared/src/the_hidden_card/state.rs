use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};
use std::time::Instant;
use strum::IntoEnumIterator;

use crate::cards::{Card, CardValue, Deck, Suit};
use crate::event::GameEvent;
pub use crate::the_hidden_card::prelude::*;
use crate::{ClientId, Player};

type PlayerSetIndex = usize;
type CalleeSetIndex = usize;
type CallerIndex = usize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayerSeat {
    pub player: Option<Player>,
    pub hands: Vec<Card>,
    coins: i32,      // 金币
    score: i32,      // 分数
    pub ready: bool, // 准备状态
    pub hands_ready: bool,
    pub player_connected: bool,
}

impl Default for PlayerSeat {
    fn default() -> Self {
        Self {
            player: None,
            hands: Vec::with_capacity(13),
            coins: 0,

            ready: false,
            hands_ready: false,
            score: 0,

            player_connected: false,
        }
    }
}

impl PlayerSeat {
    pub fn get_player(&self) -> Option<&Player> {
        self.player.as_ref()
    }
}

impl PlayerSeat {
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

    /// 检查并移除手牌
    fn remove_cards(&mut self, cards: &[Card]) -> Result<(), String> {
        let hands: HashSet<_> = self.hands.iter().collect();
        for card in cards {
            if !hands.contains(card) {
                return Err(format!("玩家没有这张牌: {:?}", card));
            }
        }

        let to_remove: HashSet<_> = cards.iter().collect();

        self.hands = self
            .hands
            .drain(..)
            .filter(|card| !to_remove.contains(card))
            .collect();

        Ok(())
    }

    /// 获取玩家某张牌型的没有的花色
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

    pub fn get_callable_cards(&self) -> Option<Vec<Card>> {
        if !self.has_full_of(CardValue::Two) {
            return Some(self.get_complement_suit(CardValue::Two));
        } else if !self.has_full_of(CardValue::Ace) {
            return Some(self.get_complement_suit(CardValue::Ace));
        } else if !self.has_full_of(CardValue::King) {
            return Some(self.get_complement_suit(CardValue::King));
        } else if !self.has_full_of(CardValue::Queen) {
            return Some(self.get_complement_suit(CardValue::Queen));
        }
        None
    }

    fn reset(&mut self) {
        self.score = 0;
        self.ready = false;
        self.hands.clear();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stage {
    PreGame,                  // 等带玩家入座
    DealCards,                // 发牌
    CallCard(PlayerSetIndex), // 叫牌
    PlayCards,                // 出牌
    Ended,                    // 游戏结束
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum GameMode {
    HiddenAllies {
        caller: usize,
        callee: usize,
        card: Card,
    }, // 暗叫组队
    OneVsThree(PlayerSetIndex), // 包牌
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::Resource))]
pub struct GameState {
    seats: [PlayerSeat; 4],
    special_card: Card,

    pub mode: Option<GameMode>,
    pub stage: Stage,

    lead: Option<usize>,                   // 第一个出牌的人
    the_hidden: Option<usize>,             // 隐藏队友
    pub current_player_seat: Option<usize>, // 当前出牌
    pub is_hidden_card_shown: bool,

    pub last_played_set_index: Option<usize>,
    pub last_played_cards: Option<Combination>,
    table_score_counter: i32,

    finished_order: VecDeque<usize>,
    // history: Vec<GameEvent>,
}

impl Default for GameState {
    fn default() -> Self {
        let mut sets: [PlayerSeat; 4] = Default::default();

        Self {
            seats: sets,
            special_card: Card::new(Suit::Spades, CardValue::Seven), //  特殊牌黑桃7喊牌

            mode: None,
            stage: Stage::PreGame,

            lead: None,
            the_hidden: None,
            current_player_seat: None,
            last_played_cards: None,
            last_played_set_index: None,
            is_hidden_card_shown: false,
            table_score_counter: 0,
            finished_order: VecDeque::new(),
            // history: Vec::new(),
        }
    }
}

impl GameState {
    pub fn get_seats(&self) -> &[PlayerSeat; 4] {
        &self.seats
    }

    pub fn set_state_by_state(&mut self, state: &GameState) {
        *self = state.clone();
    }

    pub fn get_seat_mut_by_id(&mut self, player_id: ClientId) -> Option<&mut PlayerSeat> {
        self.seats
            .iter_mut()
            .filter(|seat| seat.player.as_ref().map(|p| p.id) == Some(player_id))
            .next()
    }

    // pub fn add_history(&mut self, event: GameEvent) {
    //     self.history.push(event);
    // }
    //
    // pub fn get_history(&self) -> &[GameEvent] {
    //     &self.history
    // }

    fn get_active_set(&self) -> Option<PlayerSeat> {
        if let Some(index) = self.current_player_seat {
            Some(self.seats[index].clone())
        } else {
            None
        }
    }

    pub fn has_empty_seat(&self) -> bool {
        self.seats.iter().any(|set| set.player.is_none())
    }

    pub fn seat_is_empty(&self, index: usize) -> bool {
        self.seats[index].player.is_none()
    }

    pub fn get_seat_by_id(&self, player_id: ClientId) -> Option<&PlayerSeat> {
        self.seats
            .iter()
            .filter(|seat| seat.player.as_ref().map(|p| p.id) == Some(player_id))
            .next()
    }

    pub fn get_player_seat_index(&self, player: Player) -> Option<usize> {
        self.seats
            .iter()
            .position(|set| set.player == Some(player.clone()))
    }

    pub fn get_player_seat_index_by_id(&self, player_id: ClientId) -> Option<usize> {
        self.seats
            .iter()
            .position(|set| set.player.as_ref().map(|p| p.id) == Some(player_id))
    }

    pub fn get_empty_seat_index(&self) -> Option<usize> {
        for (index, set) in self.seats.iter().enumerate() {
            if set.player.is_none() {
                return Some(index);
            }
        }
        None
    }

    pub fn id_match_seat_index(&self, id: ClientId, index: usize) -> bool {
        if let Some(player) = &self.seats[index].player {
            return id == player.id;
        }
        false
    }

    pub fn set_seat_to_ready(&mut self, seat_index: usize) {
        self.seats[seat_index].ready = true;
    }

    pub fn is_all_ready(&self) -> bool {
        for seat in self.seats.iter() {
            if !seat.ready {
                return false;
            }
        }
        true
    }

    pub fn is_all_hands_ready(&self) -> bool {
        for seat in self.seats.iter() {
            if !seat.ready {
                return false;
            }
        }
        true
    }

    pub fn assign_seat(&mut self, player: Player, seat_index: usize) {
        self.seats[seat_index] = PlayerSeat {
            player: Some(player),
            ..Default::default()
        };
    }

    pub fn set_hands(&mut self, client_id: ClientId, hands: Vec<Card>) {
        if let Some(mut seat) = self.get_seat_mut_by_id(client_id) {
            seat.hands.clear();
            seat.hands.extend(hands);
        }
    }

    fn next_player(&mut self) {
        self.current_player_seat = match self.current_player_seat {
            Some(current) => Some((current + 1) % 4), // 循环递增
            None => Some(0), // 如果当前无玩家，从0开始, 正常情况下不会匹配到这里
        };
    }

    /// 获取可叫的牌，叫牌阶段
    fn get_callable_cards(&self) -> Option<Vec<Card>> {
        if let Stage::CallCard(caller_index) = self.stage {
            let set = &self.seats[caller_index];
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
        self.seats.iter_mut().for_each(|set| set.reset());
        self.lead = None;
        self.the_hidden = None;
        self.current_player_seat = None;
        self.last_played_cards = None;
        self.last_played_set_index = None;
        self.is_hidden_card_shown = false;
        self.finished_order.clear();

        self.mode = None;
        self.stage = Stage::PreGame;
    }

    /// 第一步：准备游戏，Stage::[PreGame, EndGame] 状态下可执行，执行后游戏进入发牌状态 Stage::DealCards
    /// 当所有玩家准备好后执行开始游戏
    /// 一轮游戏结束后需要重置玩家准备状态
    /// 初始化游戏状态
    /// 1，重置状态
    /// 2，洗牌
    /// 3，发牌
    fn prepare_game(&mut self) {
        self.reset();
        self.stage = Stage::PreGame;
    }

    /// 第一步：发牌，Stage::[PreGame] 状态下可执行，执行后游戏进入发牌状态 Stage::DealCards
    /// 当所有玩家准备好后执行开始游戏
    /// 一轮游戏结束后需要重置玩家准备状态
    /// 初始化游戏状态
    /// 1，重置状态
    /// 2，洗牌
    /// 3，发牌
    pub fn to_deal_cards_stage(&mut self) {
        self.stage = Stage::DealCards;
    }

    /// 第二步：当前状态：Stage::DealCards 执行后进入下一个状态：Stage::CallCard(caller_index)
    /// 由游戏系统发牌动作完成后执行
    /// 改状态下持有特殊牌的玩家有两个按钮： 叫牌（最多4个牌型按钮）和一个包牌按钮
    /// 其他玩家只有一个按钮： 包牌
    /// 按照优先原则
    pub fn to_call_card_stage(&mut self, caller_index: usize) {
        self.stage = Stage::CallCard(caller_index);
    }

    pub fn get_caller_id(&self) -> Option<ClientId> {
        let caller_index = self
            .seats
            .iter()
            .position(|set| set.hands.iter().any(|card| *card == self.special_card));

        let Some(caller_index) = caller_index else {
            return None;
        };

        let Some(player) = &self.seats[caller_index].player else {
            return None;
        };

        Some(player.id)
    }

    pub fn get_caller_index(&self) -> Option<usize> {
        let caller_index = self
            .seats
            .iter()
            .position(|set| set.hands.iter().any(|card| *card == self.special_card));

        caller_index
    }

    pub fn seat_hands_has_special_card(&self, index: usize) -> bool {
        self.seats[index].hands.contains(&self.special_card)
    }

    /// 第三步：当前状态：Stage::CallCard(caller_index) 执行后进入下一个状态：Stage::InGame
    /// 设置游戏模式为 GameMode::OneVsThree(player_set_index)
    /// 有玩家包牌后始游戏
    pub fn blocking_start(&mut self, player_set_index: PlayerSetIndex) {
        if !matches!(self.stage, Stage::CallCard(_)) {
            return;
        }
        self.mode = Some(GameMode::OneVsThree(player_set_index));
        self.current_player_seat = Some(player_set_index);

        // self.first = Some(player_set_index);
        self.stage = Stage::PlayCards;
    }

    /// 第三步：当前状态：Stage::CallCard(caller_index) 执行后进入下一个状态：Stage::InGame
    /// 设置游戏模式为 GameMode::HiddenAllies((caller_index, callee_index, call_card))
    /// 玩家叫牌后开始
    pub fn call_card_start(&mut self, caller_index: usize, call_card: Card) {
        if !matches!(self.stage, Stage::CallCard(_)) {
            return;
        }

        let callee_index = self
            .seats
            .iter()
            .position(|set| set.hands.iter().any(|card| *card == call_card));

        if let Some(callee_index) = callee_index {
            self.mode = Some(GameMode::HiddenAllies {
                caller: caller_index,
                callee: callee_index,
                card: call_card,
            });
            self.stage = Stage::PlayCards;
            self.current_player_seat = Some(caller_index);
        } else {
            self.mode = None;
            self.stage = Stage::Ended;
        }
    }

    pub fn play_cards(
        &mut self,
        player_set_index: PlayerSetIndex,
        cards: Vec<Card>,
    ) -> Result<(), String> {
        let combo = Combination::analyze(cards.clone());

        if combo == Combination::Invalid {
            return Err("无效牌型".to_string());
        }
        if let Some(ref last_combo) = self.last_played_cards {
            if !combo.gt(last_combo) {
                return Err("牌型太弱".to_string());
            }
        }
        // 获取玩家手牌的可变引用
        let player_set = &mut self.seats[player_set_index];

        let result = player_set.remove_cards(&cards);

        if let Err(err) = result {
            return Err(err);
        }

        // === 更新游戏状态 ===
        self.last_played_set_index = Some(player_set_index);
        self.add_table_score(&combo);
        self.last_played_cards = Some(combo);
        self.next_player();
        Ok(())
    }

    fn pass(&mut self) {
        self.next_player();
        if let (Some(current), Some(last_played)) =
            (self.current_player_seat, self.last_played_set_index)
        {
            if current == last_played {
                let player_set = self.seats.get_mut(current).unwrap();
                // 将桌面分数加到坐席分数中
                player_set.add_score(self.table_score_counter);

                // 将桌面分数清零
                self.table_score_counter = 0;
                self.last_played_set_index = None;
                self.last_played_cards = None;
            }
        }
    }

    fn add_table_score(&mut self, combo: &Combination) {
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

    #[test]
    fn test_initial_state() {
        let state = GameState::default();
        assert_eq!(state.stage, Stage::PreGame);
        assert!(state.mode.is_none());
        assert!(state.lead.is_none());
        assert!(state.the_hidden.is_none());
        assert!(state.current_player_seat.is_none());
    }

    #[test]
    fn test_pre_start_flow() {
        let mut state = GameState::default();

        // 准备阶段
        state.prepare_game();
        assert_eq!(state.stage, Stage::DealCards);

        // 检查发牌情况
        for set in &state.seats {
            assert_eq!(set.hands.len(), 13);
        }

        // 验证特殊牌存在
        let has_special = state
            .seats
            .iter()
            .any(|set| set.hands.contains(&state.special_card));
        assert!(has_special);
    }

    #[test]
    fn test_call_card_stage_transition() {
        let mut state = GameState::default();
        state.prepare_game();
        let index = state.get_caller_index();
        state.to_call_card_stage(index.unwrap().clone());
        match state.stage {
            Stage::CallCard(caller_index) => {
                // 验证叫牌者持有特殊牌
                assert!(
                    state.seats[caller_index]
                        .hands
                        .contains(&state.special_card)
                );
            },
            _ => panic!("Should be in CallCard stage"),
        }
    }

    #[test]
    fn test_blocking_start() {
        let mut state = GameState::default();
        state.prepare_game();
        let index = state.get_caller_index();
        state.to_call_card_stage(index.unwrap().clone());

        let blocker_index = 2;
        state.blocking_start(blocker_index);

        // 验证游戏模式
        match state.mode {
            Some(GameMode::OneVsThree(index)) => {
                assert_eq!(index, blocker_index);
            },
            _ => panic!("Wrong game mode"),
        }

        assert_eq!(state.stage, Stage::PlayCards);
        assert_eq!(state.current_player_seat, Some(blocker_index));
    }

    #[test]
    fn test_call_card_start() {
        let mut state = GameState::default();
        state.prepare_game();
        let index = state.get_caller_index();
        state.to_call_card_stage(index.unwrap().clone());

        let callable = state.get_callable_cards().unwrap();
        let call_card = callable.choose(&mut rng()).unwrap(); // 任意测试牌

        let Stage::CallCard(caller_index) = state.stage else {
            panic!("Not in Stage::CallCard")
        };

        state.call_card_start(caller_index, call_card.clone());

        // 验证游戏模式
        match state.mode {
            Some(GameMode::HiddenAllies {
                caller,
                callee,
                card,
            }) => {
                assert_eq!(caller, caller_index);
                assert_eq!(card, call_card.clone());
            },
            _ => panic!("Wrong game mode"),
        }
        assert_eq!(state.stage, Stage::PlayCards);
    }

    #[test]
    fn test_play_card_validation() {
        let mut state = GameState::default();
        state.stage = Stage::PlayCards; // 设置为游戏中

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
        assert!(state.play_cards(0, valid_cards).is_ok());
        assert!(state.play_cards(0, invalid_cards).is_err());
    }

    #[test]
    fn test_player_set_operations() {
        let mut set = PlayerSeat::default();

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
