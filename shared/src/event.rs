use crate::{ClientId, Player, RoomId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EndGameReason {
    PlayerLeft { player_id: ClientId },
    PlayerWon { winner: ClientId },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::Event))]
pub enum GameEvent {
    CreateRoom { player: Player },
    JoinRoom { player: Player, room_id: RoomId },
    JoinRoomOk { room_id: RoomId },

    AssignSeats { player: Player, seat_index: usize},
    BeginGame { first: ClientId },
    EndGame { reason: EndGameReason },
    PlayerJoined { player_id: ClientId, name: String },
    PlayerDisconnected { player_id: ClientId },
    PlaceTile { player_id: ClientId, at: usize },
}
