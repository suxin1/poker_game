//! 桌面展示与控制

use crate::game::assets::CardAssets;
use crate::game::widget::prelude::card_view;
use crate::prelude::*;
use crate::screens::ScreenState;
use shared::cards::Card;
use shared::event::GameEvent;
use shared::the_hidden_card::state::GameState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        handle_game_event
            .in_set(AppSystems::Update)
            .run_if(in_state(ScreenState::Gameplay)),
    );

    app.add_observer(render_table_hands);
}

fn handle_game_event(
    mut cmds: Commands,
    mut event_reader: EventReader<GameEvent>,
    state: Res<GameState>,
) {
    for event in event_reader.read() {
        match event {
            GameEvent::PlayCards(_, _) | GameEvent::SyncState(_) | GameEvent::Pass(_) => {
                let Some(combo) = state.last_played_cards.clone() else {
                    cmds.trigger(RenderTableHands(vec![]));
                    return;
                };
                cmds.trigger(RenderTableHands(combo.to_vec_cards()));
            },
            _ => {},
        }
    }
}

#[derive(Event)]
struct RenderTableHands(Vec<Card>);
fn render_table_hands(
    trigger: Trigger<RenderTableHands>,
    hands_row: Query<(Entity, &mut Children), With<TableHandsRow>>,
    mut cmds: Commands,
    card_assets: Res<CardAssets>,
) {
    let cards = &trigger.event().0;
    let (entity, children) = r!(hands_row.single());

    for child in children.iter() {
        cmds.entity(child).despawn();
    }

    cmds.entity(entity).with_children(|parent| {
        for card in cards.iter() {
            parent.spawn(card_view(
                card.clone(),
                card_assets.get_card_img(card),
                |_: Trigger<Pointer<Click>>| {},
            ));
        }
    });
}

#[derive(Component)]
struct TableHandsRow;

pub fn table_hands() -> impl Bundle {
    (
        Node {
            width: Vw(60.),
            position_type: PositionType::Absolute,
            left: Vw(20.),
            bottom: Vw(20.),
            justify_content: JustifyContent::Center,
            ..default()
        },
        TableHandsRow,
        children![]
    )
}
