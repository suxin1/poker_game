use crate::animation::ui_sprite_animation::{AnimationIndices, AnimationTimer};
use crate::game::assets::{CardAssets, IndicatorAsset, SmallCardAssets};
use crate::game::hidden_card::level::{LevelUiRoot, spawn_level};
pub use crate::game::widget::prelude::*;
use crate::prelude::*;
use crate::screens::ScreenState;
use crate::theme::palette::ThemeColor;
use bevy::ecs::observer::TriggerTargets;
use bevy::tasks::futures_lite::StreamExt;
use shared::Player;
use shared::cards::Card;
use shared::event::GameEvent;
use shared::the_hidden_card::state::{GameMode, GameState};
use std::f32::consts::PI;
use strum_macros::Display;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        handle_seat_update_event
            .in_set(AppSystems::Update)
            .run_if(in_state(ScreenState::Gameplay)),
    );
    app.add_observer(update_player_seat);

    app.add_systems(
        OnEnter(ScreenState::Gameplay),
        setup_seat_view.after(spawn_level),
    );
}

#[derive(Component)]
struct CollectedCardsCounter;

#[derive(Component)]
struct CalledCardDisplay;

#[derive(Component)]
struct TeamIndicator;

fn setup_seat_view(
    mut cmds: Commands,
    mut ui_root: Query<Entity, With<LevelUiRoot>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<IndicatorAsset>,
    card_assets: Res<CardAssets>,
    small_card_assets: Res<SmallCardAssets>,
) {
    let ui_root = r!(ui_root.single());

    let texture = assets.arrow.clone();
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 5, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 4 };

    // 2. 定义座位配置数据
    let seat_configs = [
        (
            SeatPosition::Bottom,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(-32.),
                ..default()
            },
            Quat::from_rotation_z(-PI),
            SEAT_COLOR[0],
        ),
        (
            SeatPosition::Right,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(-32.),
                ..default()
            },
            Quat::from_rotation_z(-PI),
            SEAT_COLOR[1],
        ),
        (
            SeatPosition::Top,
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(-32.),
                ..default()
            },
            Quat::IDENTITY, // 无旋转
            SEAT_COLOR[2],
        ),
        (
            SeatPosition::Left,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(-32.),
                ..default()
            },
            Quat::from_rotation_z(-PI),
            SEAT_COLOR[3],
        ),
    ];

    cmds.entity(ui_root).with_children(|parent| {
        parent
            .spawn((
                Name::new("Seat UI root"),
                Node::COLUMN_CENTER.full_size(),
                Pickable::IGNORE,
                LevelUiRoot,
                StateScoped(ScreenState::Gameplay),
            ))
            .with_children(|parent| {
                for (position, arrow_node, rotation, color) in seat_configs {
                    parent
                        .spawn(seat_view(
                            position.get_layout(),
                            position.clone(),
                            color,
                            seat_click,
                        ))
                        .with_children(|parent| {
                            let right = if matches!(position, SeatPosition::Right) {
                                Auto
                            } else {
                                Vw(-2.8)
                            };
                            let left = if matches!(position, SeatPosition::Right) {
                                Vw(-2.8)
                            } else {
                                Auto
                            };
                            parent.spawn(create_arrow_component(
                                arrow_node,
                                texture.clone(),
                                texture_atlas_layout.clone(),
                                animation_indices.clone(),
                                rotation,
                            ));
                            // 记分显示
                            parent.spawn((
                                Node {
                                    top: Vw(0.6),
                                    right,
                                    left,
                                    width: Vw(2.8),
                                    height: Vw(3.6),
                                    ..Node::ROW_CENTER.full_size().abs()
                                },
                                Visibility::Hidden,
                                CollectedCardsCounter,
                                children![
                                    (
                                        Node::DEFAULT.full_size().abs(),
                                        ImageNode {
                                            image: card_assets.back.clone(),
                                            ..default()
                                        }
                                    ),
                                    body_text("0")
                                ],
                            ));
                            // 叫牌显示
                            parent.spawn((
                                Node {
                                    bottom: Vw(0.6),
                                    right,
                                    left,
                                    width: Vw(2.8),
                                    height: Vw(3.6),
                                    ..Node::ROW_CENTER.full_size().abs()
                                },
                                Visibility::Hidden,
                                CalledCardDisplay,
                                small_card_assets.image_node(&Card::new(
                                    shared::cards::Suit::Clubs,
                                    shared::cards::CardValue::Ace,
                                )),
                            ));
                            // 队伍显示
                            parent.spawn((
                                Node {
                                    height: Vw(0.5),
                                    width: Percent(40.),
                                    bottom: if matches!(position, SeatPosition::Top) {
                                        Auto
                                    } else {
                                        Vw(-0.8)
                                    },
                                    top: if matches!(position, SeatPosition::Top) {
                                        Vw(-0.8)
                                    } else {
                                        Auto
                                    },
                                    ..Node::DEFAULT.abs()
                                },
                                TeamIndicator,
                                BorderRadius::MAX,
                                Visibility::Hidden,
                                BackgroundColor(Color::WHITE),
                            ));
                        });
                }
            });
    });
}

// ====================== 坐席显示更新 ======================

const TEAM_ONE_COLOR: Color = Color::srgba_u8(64, 150, 255, 255);
const TEAM_TWO_COLOR: Color = Color::srgba_u8(207, 19, 34, 255);

#[derive(Resource)]
struct SeatPositionMap(HashMap<SeatPosition, usize>);

#[derive(Event)]
struct RunSeatUpdate;

fn handle_seat_update_event(
    mut cmds: Commands,
    mut game_event: EventReader<GameEvent>,
    state: Res<GameState>,
    local_player: Res<Player>,
    mut is_seat_position_map_available: Local<bool>,
) {
    for event in game_event.read() {
        match event {
            GameEvent::AssignSeats { player, seat_index } => {
                // 收到本地玩家座位更新时做全量更新
                if player.id == local_player.id {
                    let local_index = c!(state.get_player_seat_index_by_id(player.id));

                    let seat_map = get_position_relative_to_local(local_index);
                    *is_seat_position_map_available = true;

                    cmds.remove_resource::<SeatPositionMap>();
                    cmds.insert_resource(SeatPositionMap(seat_map));
                }

                if *is_seat_position_map_available {
                    cmds.trigger(RunSeatUpdate);
                    // event_writer.write(LocalGameEvent::RunSeatUpdate);
                }
            },
            GameEvent::Ready { client_id: _ }
            | GameEvent::PlayCards(_, _)
            | GameEvent::Pass(_)
            | GameEvent::Blocking(_)
            | GameEvent::CallCard {
                seat_index: _,
                card: _,
            } => {
                if *is_seat_position_map_available {
                    cmds.trigger(RunSeatUpdate);
                }
            },
            GameEvent::SyncState(_) => {
                let local_index = c!(state.get_player_seat_index_by_id(local_player.id));
                let seat_map = get_position_relative_to_local(local_index);
                *is_seat_position_map_available = true;
                cmds.remove_resource::<SeatPositionMap>();
                cmds.insert_resource(SeatPositionMap(seat_map));

                if *is_seat_position_map_available {
                    cmds.trigger(RunSeatUpdate);
                }
            },
            _ => {},
        }
    }
}

fn update_player_seat(
    _: Trigger<RunSeatUpdate>,
    mut cmds: Commands,
    mut seats_query: Query<(Entity, &Children, &SeatPosition), With<SeatPosition>>,
    mut player_avatar_query: Query<Entity, With<PlayerAvatarBox>>,

    mut player_name_query: Query<&PlayerNameText>,
    mut cards_counter_query: Query<&CollectedCardsCounter>,
    mut called_card_display: Query<&CalledCardDisplay>,
    mut indicator_query: Query<&ArrowIndicator>,
    mut team_indicator: Query<&TeamIndicator>,

    mut background_query: Query<&mut BackgroundColor>,
    mut visibility_query: Query<&mut Visibility>,
    mut text_query: Query<&mut Text>,
    mut image_node_query: Query<&mut ImageNode>,
    mut children_query: Query<&Children>,
    state: Res<GameState>,
    seat_position_map: Res<SeatPositionMap>,
    small_card_assets: Res<SmallCardAssets>,
) {
    let seats_data = state.get_seats();
    for (entity, children, seat_position) in seats_query {
        let index = c!(seat_position_map.0.get(seat_position));
        let seat = &seats_data[index.clone()];
        let player_data = c!(seat.get_player());

        if let Ok(mut background_colors) = background_query.get_mut(entity) {
            background_colors.0 = if seat.ready {
                ThemeColor::INFO
            } else {
                ThemeColor::WARNING
            };
        }

        for child in children.iter() {
            if let Ok(entity) = player_avatar_query.get_mut(child) {
                if let Ok(mut background_color) = background_query.get_mut(entity) {
                    background_color.0 = SEAT_COLOR[index.clone()];
                }
            };

            if let Ok(_) = player_name_query.get(child) {
                if let Ok(mut text) = text_query.get_mut(child) {
                    **text = player_data.name.clone();
                }
            }

            if let Ok(_) = indicator_query.get(child) {
                if let Ok(mut visibility) = visibility_query.get_mut(child) {
                    *visibility =
                        Visibility::from_bool(state.current_player_seat == Some(index.clone()));
                }
            }

            if let Ok(_) = cards_counter_query.get(child) {
                if let Ok(mut visibility) = visibility_query.get_mut(child) {
                    *visibility = Visibility::from_bool(seat.score > 0);
                }
                if let Ok(children) = children_query.get(child) {
                    for child in children.iter() {
                        if let Ok(mut text) = text_query.get_mut(child) {
                            **text = seat.score.to_string();
                        }
                    }
                }
            }

            let call_card_visibility;
            let call_card_image_atlas;

            if let Ok(_) = called_card_display.get(child) {
                call_card_visibility = visibility_query.get_mut(child).unwrap();

                let image_node = image_node_query.get_mut(child).unwrap();
                call_card_image_atlas = &mut image_node.texture_atlas.unwrap();

            }

            if let Some(GameMode::HiddenAllies {
                caller,
                callee,
                card,
            }) = &state.mode
            {
                *call_card_visibility = Visibility::from_bool(*caller == *index);
                call_card_image_atlas.index = small_card_assets.get_index(&card);
                if let Ok(_) = called_card_display.get(child) {
                    if let Ok(mut visibility) = visibility_query.get_mut(child) {
                        *visibility = Visibility::from_bool(*caller == *index);
                    }
                    if let Ok(mut image_node) = image_node_query.get_mut(child) {
                        if let Some(atlas) = &mut image_node.texture_atlas {
                            atlas.index = small_card_assets.get_index(&card);
                        }
                    }
                }

                if let Ok(_) = team_indicator.get(child) {
                    let mut visibility = visibility_query.get_mut(child).unwrap();
                    let mut background_color = background_query.get_mut(child).unwrap();
                    if state.is_hidden_card_shown {
                        *visibility = Visibility::Visible;
                        if *index == *caller || *index == *callee {
                            background_color.0 = TEAM_ONE_COLOR;
                        } else {
                            background_color.0 = TEAM_TWO_COLOR;
                        }
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                }
            } else if let Some(GameMode::OneVsThree(blocking_index)) = &state.mode {
                if let Ok(_) = team_indicator.get(child) {
                    let mut visibility = visibility_query.get_mut(child).unwrap();
                    let mut background_color = background_query.get_mut(child).unwrap();
                    if state.is_hidden_card_shown {
                        *visibility = Visibility::Visible;
                        if *index == *caller || *index == *callee {
                            background_color.0 = TEAM_ONE_COLOR;
                        } else {
                            background_color.0 = TEAM_TWO_COLOR;
                        }
                    } else {
                        *visibility = Visibility::Hidden;
                    }
                }
            }
        }
    }
}

pub fn seat_click(_: Trigger<Pointer<Click>>) {
    println!("Clicked a seat!");
}

fn get_position_relative_to_local(local_index: usize) -> HashMap<SeatPosition, usize> {
    let render_mapping: [usize; 4] = [
        local_index,
        (local_index + 1) % 4,
        (local_index + 2) % 4,
        (local_index + 3) % 4,
    ];

    SeatPosition::ALL
        .into_iter()
        .zip(render_mapping.into_iter())
        .collect()
}

const SEAT_COLOR: [Color; 5] = [
    Color::srgba(0.1, 0.1, 0.5, 1.), // 蓝
    Color::srgba(0.1, 0.6, 0.1, 1.), // 绿
    Color::srgba(0.6, 0.1, 0.1, 1.), // 红
    Color::srgba(0.6, 0.4, 0.1, 1.), // 橘
    Color::srgba(0.6, 0.6, 0.6, 1.), // 橘
];

#[derive(Component, PartialEq, Eq, Hash, Clone, Display)]
pub enum SeatPosition {
    Bottom,
    Right,
    Top,
    Left,
}

impl SeatPosition {
    pub const ALL: [SeatPosition; 4] = [
        SeatPosition::Bottom,
        SeatPosition::Right,
        SeatPosition::Top,
        SeatPosition::Left,
    ];
}

impl SeatPosition {
    pub fn get_layout(&self) -> AbsolutePosition {
        match self {
            SeatPosition::Bottom => AbsolutePosition {
                bottom: Vw(1.5),
                left: Px(8.),
                top: Val::Auto,
                right: Val::Auto,
            },
            SeatPosition::Right => AbsolutePosition {
                bottom: Val::Auto,
                left: Val::Auto,
                top: Val::Auto,
                right: Px(8.),
            },
            SeatPosition::Top => AbsolutePosition {
                bottom: Val::Auto,
                left: Val::Auto,
                top: Vw(1.5),
                right: Val::Auto,
            },
            SeatPosition::Left => AbsolutePosition {
                bottom: Val::Auto,
                left: Val::Px(8.),
                top: Val::Auto,
                right: Val::Auto,
            },
        }
    }
}
