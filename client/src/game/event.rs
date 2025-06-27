//! 游戏事件处理与转发
//! 注意⚠️：GameEvent 只能从服务器发出在从 receive_event_from_server 这个函数里面转发到本地系统
//! 发送到服务器的 GameEvent 没有限制

use bevy::asset::ron::Error::Message;
use bevy::prelude::*;
use bevy_renet2::prelude::{RenetClient, ServerEvent, client_connected};
use shared::event::GameEvent;

use crate::core::AppSystems;
use crate::game::bincode::BincodeConfig;
use crate::network::MessageEvent;
use crate::prelude::{ClosePopupEvent, OpenPopupEvent};
use crate::screens::ScreenState;
use crate::theme::widget::{body_text, button_mid, card_display, text_base};
use shared::the_hidden_card::prelude::*;
use shared::{Player, Reducer};

pub(crate) fn plugin(app: &mut App) {
    app.add_event::<GameEvent>();
    app.add_systems(
        Update,
        receive_event_from_server
            .in_set(AppSystems::SyncEarly)
            .run_if(in_state(ScreenState::Title).or(in_state(ScreenState::Gameplay)))
            .run_if(resource_exists::<RenetClient>)
            .run_if(resource_exists::<Player>),
    );
    // app.add_systems(PostUpdate, receive_event_from_server.)
}

/// 接受来自服务器的事件，并将其转换为游戏事件，然后发送给本地游戏事件系统。
fn receive_event_from_server(
    mut cmds: Commands,
    mut finished: Local<bool>,
    mut client: ResMut<RenetClient>,
    mut game_event_writer: EventWriter<GameEvent>,
    mut bincode_config: Res<BincodeConfig>,
    mut next_screen: ResMut<NextState<ScreenState>>,
) {
    use GameEvent::*;
    while let Some(message) = client.receive_message(0) {
        let (event, _len): (GameEvent, usize) =
            bincode::serde::decode_from_slice(&message, bincode_config.0).unwrap();

        info!("Received event {}", event);
        match event {
            JoinRoomOk { room_id } => {
                *finished = true;
                // 收到加入房间成功事件，进入游戏屏
                next_screen.set(ScreenState::Gameplay);
                game_event_writer.write(event);
            },
            ReJoinRoomOk { room_id } => {
                next_screen.set(ScreenState::Gameplay);
                game_event_writer.write(event);
                // 关闭询问是否重新加入房间的弹窗
                cmds.trigger(ClosePopupEvent);
            },
            AskForRejoinRoom(room_id) => cmds.trigger(OpenPopupEvent {
                content_builder: Box::new(|parent| {
                    parent.spawn(card_display(
                        children![body_text("是否重新加入房间？")],
                        children![
                            button_mid("加入", rejoin_button_click),
                            button_mid("取消", cancel_rejoin_button_click)
                        ],
                    ));
                }),
                blocking: true
            }),
            _ => {
                game_event_writer.write(event);
            },
        }
    }
}

fn rejoin_button_click(_: Trigger<Pointer<Click>>, mut cmds: Commands, local_player: Res<Player>) {
    let event = GameEvent::ReJoinRoom {
        player: local_player.clone(),
    };
    cmds.trigger(MessageEvent(event));
}

fn cancel_rejoin_button_click(
    _: Trigger<Pointer<Click>>,
    mut cmds: Commands,
    local_player: Res<Player>,
) {
    let event = GameEvent::PlayerLeave(local_player.id);
    cmds.trigger(MessageEvent(event));
    cmds.trigger(ClosePopupEvent);
}
