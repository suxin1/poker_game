use crate::{ClientId, Player, RoomId};
use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::cards::Card;
use crate::error::RoomServiceError;
use crate::the_hidden_card::state::{GameState, Stage};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EndGameReason {
    PlayerLeft { player_id: ClientId },
    PlayerWon { winner: ClientId },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Display)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::Event))]
pub enum GameEvent {
    // 开发用
    RoomReset { room_id: RoomId },
    ServerReset,

    RoomError(RoomServiceError),

    ClientJustLaunched(ClientId),

    IsInRoom(ClientId),
    CreateRoom { player: Player },
    JoinRoom { player: Player, room_id: RoomId },
    JoinRoomOk { room_id: RoomId }, // 用户需要在收到该事件后再初始化游戏状态并进入游戏页面
    SyncState(GameState),

    // 重新加入房间事件
    AskForRejoinRoom(RoomId),

    ReJoinRoom { player: Player },
    ReJoinRoomOk { room_id: RoomId},

    PlayerDisconnected(ClientId),
    PlayerConnected(ClientId),
    PlayerLeave(ClientId),

    AssignSeats { player: Player, seat_index: usize },
    Ready { client_id: ClientId },

    ToDealCardStage,
    DealCards { client_id: ClientId, cards: Vec<Card>},
    DealCardsDone(ClientId),

    ToCallCardStage(usize),
    CallCard { seat_index: usize, card: Card},
    Blocking(usize),

    PlayCards(usize, Vec<Card>),
    Pass(usize),

    GameEnd(Vec<(usize, i32)>)
}
