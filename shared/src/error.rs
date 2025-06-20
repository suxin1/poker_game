use strum_macros::Display;

#[derive(Debug, PartialEq, Eq, Display)]
pub enum RoomServiceError {
    AlreadyInRoom,
    RoomNotFound,
    RoomFull,
    ClientNotInRoom
}