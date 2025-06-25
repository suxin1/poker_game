//! 负责手牌的更新，以及出牌的逻辑

use crate::game::assets::CardAssets;
use crate::game::widget::prelude::card_view;
use bevy_renet2::prelude::{ClientId, RenetClient};
use shared::Player;
use shared::cards::Card;
use shared::event::GameEvent;
use shared::the_hidden_card::prelude::*;

use crate::prelude::*;
use crate::screens::ScreenState;
use crate::theme::interaction::InteractionSelected;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        handle_update_hands_event
            .in_set(AppSystems::Update)
            .run_if(in_state(ScreenState::Gameplay)),
    );

    app.add_observer(render_hands_with_animation)
        .add_observer(render_hands_immediately);

    app.init_resource::<CardDealerMachine>().add_systems(
        Update,
        card_dealer_system
            .in_set(AppSystems::Update)
            .run_if(in_state(ScreenState::Gameplay))
            .run_if(resource_exists::<CardDealerMachine>),
    );
}

fn handle_update_hands_event(
    mut cmds: Commands,
    mut event_reader: EventReader<GameEvent>,
    game_state: Res<GameState>,
    local_player: Res<Player>,
) {
    use GameEvent::*;
    for event in event_reader.read() {
        match event {
            DealCards { client_id, cards } => {
                if local_player.id == *client_id && matches!(game_state.stage, Stage::DealCards) {
                    cmds.trigger(RenderLocalHands(cards.clone()));
                }
            },
            SyncState(_) => {
                info!("sync {:?}", game_state.get_seats());
                let index = game_state.get_player_seat_index_by_id(local_player.id);
                info!("local player index: {:?}", index);
                let seat = game_state.get_seat_by_id(local_player.id);

                if let Some(seat) = seat {
                    if !seat.hands.is_empty() {
                        info!("-----------------: {:?}", seat);
                        cmds.trigger(RenderLocalHandsImmediately(seat.hands.clone()));
                    }
                }
            },
            _ => {},
        }
    }
}

#[derive(Component)]
struct HandsRow;

#[derive(Event)]
struct RenderLocalHands(Vec<Card>);

#[derive(Event)]
struct RenderLocalHandsImmediately(Vec<Card>);

fn render_hands_with_animation(
    trigger: Trigger<RenderLocalHands>,
    mut card_dealer: ResMut<CardDealerMachine>,
) {
    let hands = &trigger.0;
    card_dealer.cards = Some(hands.clone());
    card_dealer.timer.reset();
}

fn render_hands_immediately(
    trigger: Trigger<RenderLocalHandsImmediately>,
    mut cmds: Commands,
    card_assets: Res<CardAssets>,
    mut hands_entity_query: Query<Entity, With<HandsRow>>,
) {
    let mut cards = trigger.event().0.clone();
    let entity = r!(hands_entity_query.single());
    for card in cards.iter() {
        cmds.entity(entity).with_children(|parent| {
            parent.spawn(card_view(
                card.clone(),
                card_assets.get_card_img(card.clone()),
                on_card_click,
            ));
        });
    }
}

// ====================== 发牌动画 ======================
#[derive(Resource)]
struct CardDealerMachine {
    timer: Timer,
    cards: Option<Vec<Card>>,
}

impl Default for CardDealerMachine {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.2, TimerMode::Once),
            cards: None,
        }
    }
}

fn card_dealer_system(
    mut cmds: Commands,
    time: Res<Time>,
    card_assets: Res<CardAssets>,
    mut dealer_machine: ResMut<CardDealerMachine>,
    mut hands_entity_query: Query<Entity, With<HandsRow>>,
    mut client: ResMut<RenetClient>,
    local_player: Res<Player>,
    bincode_config: Res<BincodeConfig>,
) {
    if dealer_machine.timer.tick(time.delta()).just_finished() {
        if let Some(mut cards) = dealer_machine.cards.take() {
            if let Some(card) = cards.pop() {
                dealer_machine.cards = Some(cards);
                dealer_machine.timer.reset();
                let entity = r!(hands_entity_query.single());

                cmds.entity(entity).with_children(|parent| {
                    parent.spawn(card_view(
                        card.clone(),
                        card_assets.get_card_img(card.clone()),
                        on_card_click,
                    ));
                });
            } else {
                dealer_machine.cards = None;
                client.send_message(
                    0,
                    encode_to_vec(GameEvent::DealCardsDone(local_player.id), bincode_config.0)
                        .unwrap(),
                );
            }
        }
    }
}

// ====================== 选牌系统 ======================

#[derive(Resource)]
struct SelectedCards(Vec<Card>);

fn on_card_click(
    mut trigger: Trigger<Pointer<Click>>,
    mut interaction_query: Query<&mut InteractionSelected>,
) {
    let target = trigger.target();
    let mut selected = r!(interaction_query.get_mut(target));
    selected.0 = !selected.0;
}

pub fn hands_view(children: impl Bundle) -> impl Bundle {
    (
        Node {
            width: Val::Vw(80.),
            position_type: PositionType::Absolute,
            bottom: Val::Px(16.),
            ..default()
        },
        HandsRow,
        children,
    )
}
