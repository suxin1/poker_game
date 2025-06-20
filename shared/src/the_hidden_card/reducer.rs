use crate::Reducer;
use crate::event::GameEvent;
use crate::the_hidden_card::error::GameError;
use crate::the_hidden_card::prelude::*;

impl Reducer<GameEvent, GameError> for  GameState {
    fn reduce(&mut self, event: &GameEvent) {
        use GameEvent::*;
        match event {
            AssignSeats { player, seat_index} => {
                self.assign_seat(player.clone(), seat_index.clone());
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
        // todo!()
        use GameEvent::*;
        match event {
            AssignSeats { player, seat_index} => {
                self.seat_is_empty(seat_index.clone())
            }
            _ => {
                todo!()
            }
        }
    }
}