use crate::prelude::*;
use bevy::ecs::system::IntoObserverSystem;
use shared::the_hidden_card::state::PlayerSeat;

const AVATAR_SIZE:Val = Vw(5.5);
const SET_BOX_WIDTH: Val = Vw(7.5);
const SET_BOX_HEIGHT: Val = Vh(7.5);


pub fn card_view<E, B, M, I>(set: PlayerSeat, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: Sync + IntoObserverSystem<E, B, M>,
{
    (
        Node {
            width: SET_BOX_WIDTH,
            height: SET_BOX_HEIGHT,
            ..default()
        },
        children![]
    )
}