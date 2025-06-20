use serde::{Deserialize, Serialize};
pub mod event;
pub mod cards;
pub mod the_hidden_card;
pub mod error;

pub(crate) type ClientId = u64;
pub(crate) type RoomId = u64;


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "bevy", derive(bevy_ecs::prelude::Resource))]
pub struct Player {
    pub id: ClientId,
    pub name: String,
    pub avatar: Option<String>,
}

pub trait Reducer<E, Er> {
    fn reduce(&mut self, event: &E);

    fn dispatch(&mut self, event: &E) -> Result<(), Er>;

    fn validate(&self, event: &E) -> bool;
}

