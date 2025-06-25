use strum_macros::Display;
use serde::{Deserialize, Serialize};
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize,  Clone, Display)]
pub enum RoomServiceError {
    AlreadyInRoom,
    RoomNotFound,
    RoomFull,
    ClientNotInRoom,
    ActionNotAllowed,
}