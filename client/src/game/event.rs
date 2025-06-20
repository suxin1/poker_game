use bevy::prelude::*;
use bevy_renet2::prelude::{RenetClient, ServerEvent, client_connected};
use shared::event::GameEvent;

use crate::game::bincode::BincodeConfig;
use crate::screens::ScreenState;
use shared::the_hidden_card::prelude::*;
use shared::{Player, Reducer};

pub(crate) fn plugin(app: &mut App) {
    app.add_event::<GameEvent>();
    app.add_systems(
        Update,
        receive_event_from_server
            .run_if(in_state(ScreenState::Title).or(in_state(ScreenState::Gameplay)))
            .run_if(resource_exists::<RenetClient>)
            .run_if(resource_exists::<Player>),
    );
    // app.add_systems(PostUpdate, receive_event_from_server.)
}

/// 接受来自服务器的事件，并将其转换为游戏事件，然后发送给本地游戏事件系统。
fn receive_event_from_server(
    mut finished: Local<bool>,
    mut client: ResMut<RenetClient>,
    mut game_events: EventWriter<GameEvent>,
    mut bincode_config: Res<BincodeConfig>,
    mut next_screen: ResMut<NextState<ScreenState>>,
) {
    use GameEvent::*;
    while let Some(message) = client.receive_message(0) {
        let (event, _len): (GameEvent, usize) =
            bincode::serde::decode_from_slice(&message, bincode_config.0).unwrap();

        info!("Received event {}", event);
        match event {
            JoinRoomOk {room_id} => {
                *finished = true;
                // 收到加入房间成功事件，进入游戏屏
                next_screen.set(ScreenState::Gameplay);
            }
            _ => {
                game_events.write(event);
            }
        }

    }
}
