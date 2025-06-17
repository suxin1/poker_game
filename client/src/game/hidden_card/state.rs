//! 负责初始化和销毁游戏状态

use crate::prelude::*;

use state::the_hidden_card::state::GameState;
use crate::screens::ScreenState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(ScreenState::Gameplay), init_resource);
    app.add_systems(OnExit(ScreenState::Gameplay), destory_resource);
}

fn init_resource(mut cmds: Commands) {
    cmds.insert_resource(GameState::new());
}

fn destory_resource(mut cmds: Commands) {
    cmds.remove_resource::<GameState>();
}