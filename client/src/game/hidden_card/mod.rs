use crate::prelude::*;
pub mod level;
mod state;

mod game_event;
mod player;
mod stage;
mod hands;

pub fn plugin(app: &mut App) {
    app.add_plugins((state::plugin, player::plugin, stage::plugin));
}
