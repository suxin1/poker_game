use crate::prelude::*;
use crate::screens::ScreenState;
use crate::theme::widget::button_mid;
use bevy_renet2::prelude::RenetClient;
use shared::event::GameEvent;
use shared::the_hidden_card::state::GameState;
use shared::{Player, Reducer, the_hidden_card::state::Stage};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        state_stage_control
            .in_set(AppSystems::Update)
            .run_if(in_state(ScreenState::Gameplay)),
    );

    app.add_systems(
        Update,
        handle_event_from_server
            .in_set(AppSystems::Update)
            .run_if(in_state(ScreenState::Gameplay)),
    );

    app.add_observer(show_ready_button_popup);
    // .add_observer(on_ready_botton_click);
    // .add_systems(
    //     Update,
    //     on_receive_ready_from_server
    //         .in_set(AppSystems::Update)
    //         .run_if(in_state(ScreenState::Gameplay)),
    // );
}

fn state_stage_control(
    mut cmds: Commands,
    mut event_reader: EventReader<GameEvent>,
    mut game_state: ResMut<GameState>,
    local_player: Res<Player>,
) {
    match game_state.stage {
        Stage::PreGame => {
            let seat = r!(game_state.get_seat_by_id(local_player.id));
            if !seat.ready {
                cmds.trigger(ShowReadyPopup(true));
            }
        }
        _ => {}
    }

}

/// 接收来自服务器的就绪信息,
/// 如果就绪玩家为本地玩家, 则关闭弹窗
fn handle_event_from_server(
    mut cmds: Commands,
    mut event_reader: EventReader<GameEvent>,
    local_player: Res<Player>,
) {
    for event in event_reader.read() {
        match event {
            GameEvent::Ready { client_id } => {
                if *client_id == local_player.id {
                    cmds.trigger(ClosePopupEvent);
                }
            },
            _ => {},
        }
    }
}

#[derive(Event)]
struct ShowReadyPopup(bool);
fn show_ready_button_popup(
    trigger: Trigger<ShowReadyPopup>,
    mut cmds: Commands,
    mut popup_status: Local<bool>,
) {
    let event = trigger.event();
    if event.0 && !*popup_status {
        *popup_status = true;
        cmds.trigger(OpenPopupEvent {
            content_builder: Box::new(|parent| {
                parent.spawn(button_mid("开始", on_ready_botton_click));
            }),
        });
    }
}

/// 发送玩家就绪信息到服务器
fn on_ready_botton_click(
    _: Trigger<Pointer<Click>>,
    mut client: ResMut<RenetClient>,
    local_player: Res<Player>,
    bincode_config: Res<BincodeConfig>,
) {
    let event = GameEvent::Ready {
        client_id: local_player.id,
    };
    if client.is_connected() {
        client.send_message(0, encode_to_vec(event, bincode_config.0).unwrap());
    }
}
