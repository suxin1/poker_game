use bevy::input::keyboard::Key::Close;
use bevy_renet2::prelude::RenetClient;

use crate::game::widget::prelude::*;
use crate::prelude::*;
use crate::screens::ScreenState;
use crate::theme::widget::button_mid;

use crate::game::assets::CardAssets;
use crate::game::hidden_card::hands::{HandsRow, SelectedCards};
use shared::cards::Card;
use shared::event::GameEvent;
use shared::the_hidden_card::state::GameState;
use shared::{Player, Reducer, the_hidden_card::state::Stage};
use shared::the_hidden_card::prelude::Combination;
use crate::network::MessageEvent;
use crate::theme::interaction::InteractionSelected;

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
            .in_set(AppSystems::HandleServerEvents)
            .run_if(in_state(ScreenState::Gameplay)),
    );

    app.add_observer(show_ready_button_popup);

    app.add_observer(show_call_card_popup);

    app.add_observer(show_play_card_popup);
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
    mut is_play_card_popup_showed: Local<bool>,
) {
    for event in event_reader.read() {
        match event {
            // GameEvent::RoundUp { stage } => {
            //     game_state.stage = *stage;
            //     *is_ready_event_send = false;
            //     *is_call_card_event_send = false;
            //     *is_play_card_popup_showed = false;
            // },
            GameEvent::PlayCards(index, _) => {
                // 出牌后重置弹窗状态，准备下一次出牌
                let seat_index = c!(game_state.get_player_seat_index_by_id(local_player.id));
                if seat_index == *index {
                    *is_play_card_popup_showed = false;
                }
            }
            _ => {},
        }
    }

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
        Stage::PlayCards => {
            let current_play_index = r!(game_state.current_player_seat);
            let local_player_index = r!(game_state.get_player_seat_index_by_id(local_player.id));
            if local_player_index == current_play_index && !*is_play_card_popup_showed {
                cmds.trigger(ShowPlayCardPopup);
                *is_play_card_popup_showed = true;
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
    state: Res<GameState>,
) {
    for event in event_reader.read() {
        match event {
            GameEvent::Ready { client_id } => {
                if *client_id == local_player.id {
                    cmds.trigger(ClosePopupEvent);
                }
            },
            GameEvent::Blocking(_)
            | GameEvent::CallCard {
                seat_index: _,
                card: _,
            } => {
                // 叫牌或抢牌后关闭弹窗
                cmds.trigger(ClosePopupEvent);
            },
            GameEvent::PlayCards(index, _) => {
                if state.id_match_seat_index(local_player.id, *index) {
                    cmds.trigger(ClosePopupEvent);
                }
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
            blocking: true,
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
                    .spawn((Node {
                        justify_content: JustifyContent::SpaceEvenly,
                        column_gap: Vw(0.5),
                        padding: UiRect {
                            top: Vw(0.5),
                            ..default()
                        },
                        ..default()
                    },))
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
        blocking: true,
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
    let event = GameEvent::CallCard {
        seat_index,
        card: card.0.clone(),
    };

    if client.is_connected() {
        client.send_message(0, encode_to_vec(event, bincode_config.0).unwrap());
    }
}

// ====================== 出牌 ======================

#[derive(Event)]
struct ShowPlayCardPopup;

fn show_play_card_popup(
    _: Trigger<ShowPlayCardPopup>,
    mut cmds: Commands,
    mut selected_cards: ResMut<SelectedCards>,
    state: Res<GameState>,
) {
    let state = state.clone();
    cmds.trigger(OpenPopupEvent {
        blocking: false,
        content_builder: Box::new(move |parent| {
            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Vw(1.0),
                    ..default()
                },))
                .with_children(|parent| {
                    if state.last_played_cards.is_some() {
                        parent.spawn(button_mid("不要", on_pass_button_click));
                    }
                    parent.spawn(button_mid("出牌", on_play_cards_button_click));
                });
        }),
    });
}

fn on_pass_button_click(trigger: Trigger<Pointer<Click>>, mut cmds: Commands) {
    cmds.trigger(ClosePopupEvent);
}

fn on_play_cards_button_click(
    _: Trigger<Pointer<Click>>,
    mut cmds: Commands,
    hands_query: Query<&Children, With<HandsRow>>,
    card_data_query: Query<(&CardData, &InteractionSelected)>,
    state: Res<GameState>,
    local_player: Res<Player>
) {
    let children = r!(hands_query.single());
    let mut cards = Vec::new();
    for child in children.iter() {
        let (card_data, select) = r!(card_data_query.get(child));
        if select.0 {
            cards.push(card_data.0.clone());
        }
    }
    let combination = Combination::analyze(cards.clone());
    if matches!(combination, Combination::Invalid) {
        return;
    }
    let index = r!(state.get_player_seat_index_by_id(local_player.id));
    cmds.trigger(MessageEvent(GameEvent::PlayCards(index, cards)));
}
