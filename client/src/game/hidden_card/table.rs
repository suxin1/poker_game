//! 桌面展示与控制

use crate::game::assets::CardAssets;
use crate::game::hidden_card::seat::CARD_WIDTH;
use crate::game::widget::prelude::{CARD_HEIGHT, card_view};
use crate::prelude::*;
use crate::screens::ScreenState;
use bevy::ui::*;
use shared::cards::Card;
use shared::event::GameEvent;
use shared::the_hidden_card::state::GameState;
use std::ops::Deref;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        handle_game_event
            .in_set(AppSystems::Update)
            .run_if(in_state(ScreenState::Gameplay)),
    );

    app.add_observer(render_table_hands);
    app.add_observer(update_table_counter);
}

fn handle_game_event(
    mut cmds: Commands,
    mut event_reader: EventReader<GameEvent>,
    state: Res<GameState>,
) {
    for event in event_reader.read() {
        match event {
            GameEvent::PlayCards(_, _) | GameEvent::SyncState(_) | GameEvent::Pass(_) => {
                if let Some(combo) = state.last_played_cards.clone() {
                    cmds.trigger(RenderTableHands(combo.to_vec_cards()));
                } else {
                    cmds.trigger(RenderTableHands(vec![]));
                }
                cmds.trigger(UpdateTableCounter);
            },
            _ => {},
        }
    }
}

#[derive(Event)]
struct RenderTableHands(Vec<Card>);
/// ### 渲染上一个玩家压下的牌组
/// ⚠️注意这里不能使用 `Query<(Entity, &Children), With<TableHandsRow>>`
/// 因为下面这段代码在移除children后，会导致[`Children`]会从Entity中移除，
/// 导致下一次查询[`TableHandsRow`]相关[`Entity`]失败。
/// ```
/// for child in children.iter() {
///     cmds.entity(child).despawn();
/// }
/// ```
fn render_table_hands(
    trigger: Trigger<RenderTableHands>,
    hands_row: Query<Entity, With<TableHandsRow>>,
    children_query: Query<&Children>,
    mut cmds: Commands,
    card_assets: Res<CardAssets>,
) {
    let cards = &trigger.event().0;
    let entity = r!(hands_row.single());

    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            cmds.entity(child).despawn();
        }
    }

    cmds.entity(entity).with_children(|parent| {
        for card in cards.iter() {
            parent.spawn((
                Node {
                    width: CARD_WIDTH,
                    height: CARD_HEIGHT,
                    ..default()
                },
                ImageNode {
                    // 直接添加ImageNode组件
                    image: card_assets.get_card_img(card),
                    ..default()
                },
            ));
        }
    });
}

#[derive(Event)]
struct UpdateTableCounter;

fn update_table_counter(
    _: Trigger<UpdateTableCounter>,
    state: Res<GameState>,
    mut counter_query: Query<(&mut Visibility, &Children), With<TableCardCounter>>,
    mut image_node_query: Query<&mut ImageNode>,
    mut text_query: Query<&mut Text>,
    mut image_updated: Local<bool>,
    card_assets: Res<CardAssets>,
) {
    let (mut visible, children) = r!(counter_query.single_mut());
    let visible_bool = state.table_score_counter > 0;

    *visible = if visible_bool {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    for child in children.iter() {
        if !*image_updated {
            if let Ok(mut image) = image_node_query.get_mut(child) {
                image.image = card_assets.back.clone();
                *image_updated = true;
            }
        }
        if let Ok(mut text) = text_query.get_mut(child) {
            **text = state.table_score_counter.to_string();
        }
    }
}

#[derive(Component)]
struct TableHandsRow;

#[derive(Component)]
struct TableCardCounter;

const TABLE_HANDS_BOTTOM_DISTANCE: Val = Vw(20.);
const TABLE_HANDS_LEFT_DISTANCE: Val = Vw(20.);

pub fn table() -> impl Bundle {
    info!("render table");
    (
        Node {
            ..Node::DEFAULT.full_size().abs()
        },
        Pickable::IGNORE,
        children![
            (
                Node {
                    width: Vw(60.),
                    height: CARD_HEIGHT,
                    position_type: PositionType::Absolute,
                    left: TABLE_HANDS_LEFT_DISTANCE,
                    bottom: TABLE_HANDS_BOTTOM_DISTANCE,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                TableHandsRow,
            ),
            (
                Node {
                    height: CARD_HEIGHT,
                    width: CARD_WIDTH,
                    position_type: PositionType::Absolute,
                    bottom: TABLE_HANDS_BOTTOM_DISTANCE.try_add(CARD_HEIGHT).unwrap(),
                    left: TABLE_HANDS_LEFT_DISTANCE,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![
                    (
                        Node {
                            ..Node::DEFAULT.full_size().abs()
                        },
                        ImageNode { ..default() },
                    ),
                    (
                        Node {
                            // justify_content: JustifyContent::Center,
                            // align_items: AlignItems::Center,
                            // ..Node::DEFAULT.full_size().abs()
                            ..default()
                        },
                        body_text("0"),
                    )
                ],
                TableCardCounter,
                Visibility::Hidden,
            )
        ],
    )
}
