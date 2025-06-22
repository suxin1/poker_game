use crate::game::hidden_card::level::LevelUiRoot;
pub use crate::game::widget::prelude::*;
use crate::prelude::*;
use crate::screens::ScreenState;
use bevy::reflect::Array;
use shared::Player;
use shared::event::GameEvent;
use shared::the_hidden_card::state::GameState;
use url::Position;

use strum_macros::Display;
use crate::game::hidden_card::game_event::LocalGameEvent;
use crate::theme::palette::ThemeColor;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        handle_seat_update_event
            .in_set(AppSystems::Update)
            .run_if(in_state(ScreenState::Gameplay)),
    );
    app.add_observer(update_player_seat);
}

#[derive(Resource)]
struct SeatPositionMap(HashMap<SeatPosition, usize>);

#[derive(Event)]
struct RunSeatUpdate;

fn handle_seat_update_event(
    mut cmds: Commands,
    mut game_event: EventReader<GameEvent>,
    state: Res<GameState>,
    local_player: Res<Player>,
    mut is_seat_position_map_available: Local<bool>
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
            GameEvent::Ready {client_id} => {
                if *is_seat_position_map_available {
                    cmds.trigger(RunSeatUpdate);
                }
            }
            _ => {},
        }
    }
}

fn update_player_seat(
    _: Trigger<RunSeatUpdate>,
    mut seats_query: Query<(Entity, &Children, &SeatPosition), With<SeatPosition>>,
    mut player_avatar: Query<Entity, With<PlayerAvatarBox>>,
    mut player_name_text_query: Query<&mut Text, With<PlayerNameText>>,
    mut background_query: Query<&mut BackgroundColor>,
    state: Res<GameState>,
    seat_position_map: Res<SeatPositionMap>,
) {
    info!("Update seat view");
    let seats_data = state.get_seats();
    for (entity, children, seat_position) in seats_query.iter_mut() {
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
            if let Ok(mut text) = player_name_text_query.get_mut(child) {
                **text = player_data.name.clone();
            };
            if let Ok(entity) = player_avatar.get_mut(child) {
                if let Ok(mut background_color) = background_query.get_mut(entity) {
                    background_color.0 = SEAT_COLOR[index.clone()];
                }
            };
        }
    }
}

pub fn seats_view() -> impl Bundle {
    (
        Name::new("Game UI root"),
        Node::COLUMN_CENTER.full_size(),
        Pickable::IGNORE,
        LevelUiRoot,
        StateScoped(ScreenState::Gameplay),
        children![
            seat_view(
                SeatPosition::Bottom.get_layout(),
                SeatPosition::Bottom,
                SEAT_COLOR[4],
                seat_click
            ),
            seat_view(
                SeatPosition::Right.get_layout(),
                SeatPosition::Right,
                SEAT_COLOR[4],
                seat_click
            ),
            seat_view(
                SeatPosition::Top.get_layout(),
                SeatPosition::Top,
                SEAT_COLOR[4],
                seat_click
            ),
            seat_view(
                SeatPosition::Left.get_layout(),
                SeatPosition::Left,
                SEAT_COLOR[4],
                seat_click
            ),
        ],
    )
}

pub fn seat_click(_: Trigger<Pointer<Click>>) {
    println!("Clicked a card!");
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

struct ColorPlatte;

impl ColorPlatte {
    const BLUE: Color = Color::srgba(0.1, 0.1, 0.5, 1.);
    // const READY_COLOR: Color = Color::srgba()
}

#[derive(Component, PartialEq, Eq, Hash, Display)]
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
                bottom: Px(8.),
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
                top: Px(8.),
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
