use crate::prelude::*;
pub mod level;
mod state;

mod game_event;
mod hands;
mod seat;
mod stage;
mod table;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        state::plugin,
        seat::plugin,
        stage::plugin,
        hands::plugin, // 本地玩家手牌控制
        table::plugin,
    ));
}
