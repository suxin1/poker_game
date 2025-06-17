use serde::{Deserialize, Serialize};
use crate::state::PlayerId;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EndGameReason {
    PlayerLeft { player_id: PlayerId },
    PlayerWon { winner: PlayerId },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::Event))]
pub enum GameEvent {
    BeginGame { first: PlayerId },
    EndGame { reason: crate::event::EndGameReason },
    PlayerJoined { player_id: PlayerId, name: String },
    PlayerDisconnected { player_id: PlayerId },
    PlaceTile { player_id: PlayerId, at: usize },
}
