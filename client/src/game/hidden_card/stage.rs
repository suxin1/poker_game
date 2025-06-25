use bevy_renet2::prelude::RenetClient;

use crate::game::widget::prelude::*;
use crate::prelude::*;
use crate::screens::ScreenState;
use crate::theme::widget::button_mid;

use crate::game::assets::CardAssets;
use shared::cards::Card;
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

    app.add_observer(show_call_card_popup);
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
    mut is_ready_event_send: Local<bool>, // 是否发送了准备事件
    mut is_call_card_event_send: Local<bool>, // 是否发送了叫牌事件
) {
    match game_state.stage {
        Stage::PreGame => {
            let seat = r!(game_state.get_seat_by_id(local_player.id));
            if !seat.ready && !*is_ready_event_send {
                cmds.trigger(ShowReadyPopup(true));
                *is_ready_event_send = true;
            }
        },
        Stage::CallCard(index) => {
            let seat_index = r!(game_state.get_player_seat_index_by_id(local_player.id));
            let seat = &game_state.get_seats()[seat_index];
            if seat.hands_ready && !*is_call_card_event_send {
                if seat_index == index {
                    let cards = r!(seat.get_callable_cards());
                    cmds.trigger(ShowCallCardPopup(cards));
                } else {
                    cmds.trigger(ShowCallCardPopup(vec![]));
                }
                *is_call_card_event_send = true;
            }
        },
        _ => {},
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
            GameEvent::Blocking(_) | GameEvent::CallCard {seat_index: _, card: _} => {
                cmds.trigger(ClosePopupEvent);
            }
            _ => {},
        }
    }
}

// ====================== 准备 ======================

#[derive(Event)]
struct ShowReadyPopup(bool);
fn show_ready_button_popup(trigger: Trigger<ShowReadyPopup>, mut cmds: Commands) {
    let event = trigger.event();
    if event.0 {
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

// ====================== 叫牌 ======================

/// 显示叫牌的弹窗
#[derive(Event)]
struct ShowCallCardPopup(Vec<Card>);

fn show_call_card_popup(
    trigger: Trigger<ShowCallCardPopup>,
    mut cmds: Commands,
    card_assets: Res<CardAssets>,
) {
    let cards = trigger.event().0.clone();
    let card_assets = card_assets.clone();
    cmds.trigger(OpenPopupEvent {
        content_builder: Box::new(move |parent| {
            parent.spawn(button_mid("包", on_blocking_botton_click));
            if !cards.is_empty() {
                parent
                    .spawn((
                        Node {
                            justify_content: JustifyContent::SpaceEvenly,
                            ..default()
                        },
                        Text::new("sssss"),
                    ))
                    .with_children(|parent| {
                        for card in cards.iter() {
                            parent.spawn(card_view(
                                card.clone(),
                                card_assets.get_card_img(card.clone()),
                                on_call_card_click,
                            ));
                        }
                    });
            }
        }),
    });
}

/// 发送包牌事件到服务器
fn on_blocking_botton_click(
    _: Trigger<Pointer<Click>>,
    mut client: ResMut<RenetClient>,
    local_player: Res<Player>,
    game_state: Res<GameState>,
    bincode_config: Res<BincodeConfig>,
) {
    let seat_index = r!(game_state.get_player_seat_index_by_id(local_player.id));
    let event = GameEvent::Blocking(seat_index);
    if client.is_connected() {
        client.send_message(0, encode_to_vec(event, bincode_config.0).unwrap());
    }
}

fn on_call_card_click(
    trigger: Trigger<Pointer<Click>>,
    mut client: ResMut<RenetClient>,
    local_player: Res<Player>,
    game_state: Res<GameState>,
    bincode_config: Res<BincodeConfig>,
    card_data_query: Query<&CardData>,
) {
    let seat_index = r!(game_state.get_player_seat_index_by_id(local_player.id));
    let target = trigger.target();
    let card = r!(card_data_query.get(target));
    let event = GameEvent::CallCard { seat_index: seat_index, card: card.0.clone() };

    if client.is_connected() {
        client.send_message(0, encode_to_vec(event, bincode_config.0).unwrap());
    }
}
