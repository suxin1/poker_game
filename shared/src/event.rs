use crate::{ClientId, Player, RoomId};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EndGameReason {
    PlayerLeft { player_id: ClientId },
    PlayerWon { winner: ClientId },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Display)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::Event))]
pub enum GameEvent {
    CreateRoom { player: Player },
    JoinRoom { player: Player, room_id: RoomId },
    RoomReset { room_id: RoomId},
    JoinRoomOk { room_id: RoomId }, // 用户需要在收到该事件后再初始化游戏状态并进入游戏

    AssignSeats { player: Player, seat_index: usize},
    BeginGame { first: ClientId },
    EndGame { reason: EndGameReason },
    PlayerJoined { player_id: ClientId, name: String },
    PlayerDisconnected { player_id: ClientId },
    PlaceTile { player_id: ClientId, at: usize },
}
