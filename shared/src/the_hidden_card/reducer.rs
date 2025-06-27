use log::{error, info};
use tiny_bail::prelude::*;

use crate::Reducer;
use crate::event::GameEvent;
use crate::the_hidden_card::error::GameError;
use crate::the_hidden_card::prelude::*;
use crate::the_hidden_card::state::Stage;

impl Reducer<GameEvent, GameError> for GameState {
    fn reduce(&mut self, event: &GameEvent) {
        use GameEvent::*;
        match event {
            AssignSeats { player, seat_index } => {
                self.assign_seat(player.clone(), seat_index.clone());
            },
            Ready { client_id } => {
                let Some(seat_index) = self.get_player_seat_index_by_id(client_id.clone()) else {
                    return;
                };
                self.set_seat_to_ready(seat_index);
            },
            ToDealCardStage => {
                self.to_deal_cards_stage();
            },
            DealCards { client_id, cards } => {
                self.set_hands(client_id.clone(), cards.clone());
            },
            DealCardsDone(client_id) => {
                let seat = r!(self.get_seat_mut_by_id(client_id.clone()));
                seat.hands.sort_by(|a, b| b.cmp(a));
                seat.hands_ready = true;
            },
            ToCallCardStage(idx) => {
                self.to_call_card_stage(idx.clone());
            },
            CallCard { seat_index, card } => {
                self.call_card_start(seat_index.clone(), card.clone());
            },
            Blocking(index) => {
                self.blocking_start(index.clone());
            },
            PlayCards(seat_index, cards) => {
                self.play_cards(seat_index.clone(), cards.clone());
            },
            Pass(_) => {
                self.pass();
            },
            PlayerDisconnected(client_id) => {
                let seat = r!(self.get_seat_mut_by_id(client_id.clone()));
                seat.player_connected = false;
            },
            PlayerConnected(client_id) => {
                let seat = r!(self.get_seat_mut_by_id(client_id.clone()));
                seat.player_connected = true;
            },
            SyncState(state) => {
                self.set_state_by_state(state);
            },
            _ => {},
        }
        // self.add_history(event.clone());
    }

    fn dispatch(&mut self, event: &GameEvent) -> Result<(), GameError> {
        if !self.validate(&event) {
            return Err(GameError::InvalidEvent);
        }
        Ok(())
    }

    fn validate(&self, event: &GameEvent) -> bool {
        use GameEvent::*;
        match event {
            AssignSeats { player, seat_index } => self.seat_is_empty(seat_index.clone()),
            Ready { client_id } => {
                let Some(seat) = self.get_seat_by_id(client_id.clone()) else {
                    return false;
                };
                if seat.ready {
                    // 已经就绪，无须二次确认
                    return false;
                }
                true
            },
            ToDealCardStage => self.stage == Stage::PreGame,
            DealCards { client_id, cards } => cards.len() == 13,
            DealCardsDone(client_id) => {
                let Some(seat) = self.get_seat_by_id(client_id.clone()) else {
                    return false;
                };
                if seat.hands_ready {
                    return false;
                }
                true
            },
            ToCallCardStage(index) => {
                self.stage == Stage::DealCards && self.seat_hands_has_special_card(index.clone())
            },
            Blocking(index) => {
                matches!(self.stage, Stage::CallCard(_))
            },
            CallCard { seat_index, card } => self.stage == Stage::CallCard(seat_index.clone()),
            PlayCards(seat_index, cards) => {
                let pre_condition = matches!(self.stage, Stage::PlayCards)
                    && Some(seat_index.clone()) == self.current_player_seat;
                if !pre_condition {
                    return false;
                }
                let Ok(_) = self.can_play_cards(cards) else {
                    return false;
                };
                true
            },
            Pass(seat_index) => {
                matches!(self.stage, Stage::PlayCards)
                    && Some(seat_index.clone()) == self.current_player_seat
            },
            PlayerDisconnected(_) => true,
            SyncState(_) => true,
            _ => {
                error!(target: "Game state", "Not implement {}", event);
                false
            },
        }
    }
}
