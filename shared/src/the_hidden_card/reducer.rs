use log::info;
use crate::Reducer;
use crate::event::GameEvent;
use crate::the_hidden_card::error::GameError;
use crate::the_hidden_card::prelude::*;
use crate::the_hidden_card::state::Stage;

impl Reducer<GameEvent, GameError> for  GameState {
    fn reduce(&mut self, event: &GameEvent) {
        use GameEvent::*;
        match event {
            AssignSeats { player, seat_index} => {
                self.assign_seat(player.clone(), seat_index.clone());
            },
            Ready {client_id} => {
                let Some(seat_index) = self.get_player_seat_index_by_id(client_id.clone()) else {
                    return;
                };
                self.set_seat_to_ready(seat_index);
            },
            ToDealCardStage => {
                self.to_deal_cards_stage();
            }
            DealCards {client_id, cards} => {

            }
            _ => {}
        }
        self.add_history(event.clone());
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
            AssignSeats { player, seat_index} => {
                self.seat_is_empty(seat_index.clone())
            }
            Ready { client_id} => {
                let Some(seat) = self.get_seat_by_id(client_id.clone()) else {
                   return false;
                };
                if seat.ready {
                    // 已经就绪，无须二次确认
                    return false;
                }
                true
            }
            ToDealCardStage => {
                self.stage == Stage::PreGame
            }
            DealCards { client_id, cards } => {
                cards.len() == 13
            }
            _ => {
                info!("Event not handle: {}", event);
                todo!()
            }
        }
    }
}