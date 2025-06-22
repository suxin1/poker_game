use crate::{ClientId, Player, RoomId};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::cards::Card;
use crate::error::RoomServiceError;
use crate::the_hidden_card::state::Stage;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EndGameReason {
    PlayerLeft { player_id: ClientId },
    PlayerWon { winner: ClientId },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Display)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::Event))]
pub enum GameEvent {
    RoomError(RoomServiceError),

    CreateRoom { player: Player },
    JoinRoom { player: Player, room_id: RoomId },
    RoomReset { room_id: RoomId },
    JoinRoomOk { room_id: RoomId }, // 用户需要在收到该事件后再初始化游戏状态并进入游戏

    AssignSeats { player: Player, seat_index: usize },
    Ready { client_id: ClientId },
    ToDealCardStage,
    DealCards { client_id: ClientId, cards: Vec<Card>},
    
}
