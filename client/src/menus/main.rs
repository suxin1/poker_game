//! The main menu (seen on the title screen).

use bevy_renet2::prelude::RenetClient;
use bincode::serde::encode_to_vec;

use crate::prelude::*;

use shared::Player;
use shared::event::GameEvent;

#[cfg(target_family = "wasm")]
use crate::theme::interaction::InteractionDisabled;

use crate::game::bincode::BincodeConfig;
use crate::prelude::{ClosePopupEvent, OpenPopupEvent};
use crate::{asset_tracking::ResourceHandles, menus::Menu, screens::ScreenState, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Main Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Main),
        // #[cfg(not(target_family = "wasm"))]
        children![
            widget::button("开始", enter_loading_or_gameplay_screen),
            widget::button("设置", open_settings_menu),
            widget::button("打开弹窗", open_popup),
            (
                widget::button("退出", exit_app),
                #[cfg(target_family = "wasm")]
                InteractionDisabled(true)
            )
        ],
        // #[cfg(target_family = "wasm")]
        // children![
        //     widget::button("开始", enter_loading_or_gameplay_screen),
        //     widget::button("设置", open_settings_menu),
        // ],
    ));
}

fn enter_loading_or_gameplay_screen(
    _: Trigger<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<ScreenState>>,
    mut client: ResMut<RenetClient>,
    player: Res<Player>,
    bincode_config: Res<BincodeConfig>,
) {
    info!("Entering loading or gameplay screen.");
    if resource_handles.is_all_done() {
        // next_screen.set(ScreenState::Gameplay);
        let event = GameEvent::JoinRoom {
            player: player.clone(),
            room_id: 0,
        };
        client.send_message(0, encode_to_vec(&event, bincode_config.0).unwrap());
    } else {
        next_screen.set(ScreenState::Loading);
    }
}

fn open_settings_menu(
    _: Trigger<Pointer<Click>>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut client: ResMut<RenetClient>,
    bincode_config: Res<BincodeConfig>,
) {
    #[cfg(feature = "dev")]
    {
        info!("发送重置游戏房间事件到服务器");
        let event = GameEvent::RoomReset { room_id: 0 };
        client.send_message(0, encode_to_vec(&event, bincode_config.0).unwrap());
    }
    next_menu.set(Menu::Settings);
}

fn open_popup(_: Trigger<Pointer<Click>>, mut cmds: Commands) {
    cmds.trigger(OpenPopupEvent {
        content_builder: Box::new(|parent| {
            parent.spawn((
                Node {
                    width: Vw(50.),
                    height: Vw(30.),
                    padding: UiRect::all(Vw(3.)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                BackgroundColor(Color::WHITE),
                BorderRadius::all(Vw(5.0)),
                children![
                    (
                        Node {
                            width: Percent(100.),
                            flex_grow: 1.,
                            ..default()
                        },
                    ),
                    (
                        Node {
                            width: Percent(100.),
                            // height: Percent(20.),
                            justify_content: JustifyContent::End,
                            ..default()
                        },
                        children![widget::button_mid("关闭", close_popup)],
                    )
                ],
            ));
        }),
    });
}

fn close_popup(_: Trigger<Pointer<Click>>, mut cmds: Commands) {
    cmds.trigger(ClosePopupEvent);
}

// #[cfg(not(target_family = "wasm"))]
fn exit_app(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    #[cfg(not(target_family = "wasm"))]
    app_exit.write(AppExit::Success);
}
