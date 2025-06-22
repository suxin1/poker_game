use crate::prelude::*;
pub mod level;
mod state;

mod game_event;
mod player;
mod stage;

pub fn plugin(app: &mut App) {
    app.add_plugins((state::plugin, player::plugin));
}
