use bevy::prelude::*;
use bevy_renet2::prelude::{RenetClient, ServerEvent, client_connected};
use strum_macros::Display;
use shared::event::GameEvent;

use crate::game::bincode::BincodeConfig;
use crate::screens::ScreenState;
use shared::Reducer;
use shared::the_hidden_card::prelude::*;
use crate::prelude::{Deserialize, Serialize};

pub(crate) fn plugin(app: &mut App) {
    app.add_event::<LocalGameEvent>();
    // app.add_systems(
    //     Update,
    //     receive_event_from_server
    //         .run_if(in_state(ScreenState::Gameplay))
    //         .run_if(resource_exists::<GameState>),
    // );
    // app.add_systems(PostUpdate, receive_event_from_server.)
}

#[derive(Debug, Clone, Event, PartialEq, Eq, Display)]
pub enum LocalGameEvent {
    RunSeatUpdate,
}
