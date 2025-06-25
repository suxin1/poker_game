//! 负责初始化和销毁游戏状态

use crate::prelude::*;
use crate::screens::ScreenState;

use shared::event::GameEvent;
use shared::the_hidden_card::prelude::*;
use shared::Reducer;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(ScreenState::Gameplay), init_resource);
    app.add_systems(OnExit(ScreenState::Gameplay), destory_resource);

    app.add_systems(
        Update,
        update_state
            .in_set(AppSystems::UpdateState)
            .run_if(in_state(ScreenState::Gameplay))
            .run_if(resource_exists::<GameState>),
    );
}

fn init_resource(mut cmds: Commands) {
    cmds.insert_resource(GameState::default());
}

fn destory_resource(mut cmds: Commands) {
    cmds.remove_resource::<GameState>();
}

/// 接受来自系统转发的服务器事件，并更新状态。
fn update_state(
    mut game_state: ResMut<GameState>,
    mut sys_events: EventReader<GameEvent>,
) {
    use GameEvent::*;
    for event in sys_events.read() {
        // 这里我们相信服务器给我们的事件是合法的，所以直接应用到游戏状态上, 非游戏状态更新事件会被GameState忽略。
        game_state.reduce(&event);
    }
}
