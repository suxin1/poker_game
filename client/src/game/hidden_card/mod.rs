use crate::prelude::*;
pub mod level;
mod state;

mod game_event;
mod player;

pub fn plugin(app: &mut App) {
    app.add_plugins((state::plugin, player::plugin));
}
