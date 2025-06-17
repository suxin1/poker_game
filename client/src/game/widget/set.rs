use crate::prelude::*;
use bevy::ecs::system::IntoObserverSystem;
use state::the_hidden_card::state::PlayerSet;

const AVATAR_SIZE:Val = Vw(5.5);

pub fn card_view<E, B, M, I>(set: PlayerSet, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: Sync + IntoObserverSystem<E, B, M>,
{
    (
        Node {
            width: AVATAR_SIZE,
            height: AVATAR_SIZE,
            ..default()
        },
        BorderRadius::MAX,
        Patch(|entity| {
            entity.observe(action);
        })
    )
}