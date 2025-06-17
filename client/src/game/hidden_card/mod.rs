use crate::prelude::*;
mod state;
pub mod game;

pub fn plugin(app: &mut App) {
    app.add_plugins(state::plugin);
}

