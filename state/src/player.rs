use serde::{Deserialize, Serialize};

pub(crate) type PlayerId = u64;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Player {
    id: PlayerId,
    name: String,
    avatar: String,
}