//! 发送重置游戏房间事件到服务器
//! 服务器会将房间重新初始化

use crate::prelude::*;
use bevy_renet2::prelude::RenetClient;
use shared::Player;
use shared::event::GameEvent;
use crate::screens::ScreenState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        send_reset_game_event
            .run_if(in_state(ScreenState::Title))
            .run_if(resource_added::<RenetClient>)
            .in_set(AppSystems::Update),
    );
}

fn send_reset_game_event(mut client: ResMut<RenetClient>, bincode_config: Res<BincodeConfig>) {

}
