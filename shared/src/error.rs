#[derive(Debug, PartialEq, Eq)]
pub enum RoomServiceError {
    AlreadyInRoom,
    RoomNotFound,
    RoomFull
}