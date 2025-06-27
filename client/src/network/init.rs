use crate::prelude::*;
use std::ops::Deref;

#[cfg(not(target_arch = "wasm32"))]
use crate::network::native::create_renet_client;
use crate::screens::ScreenState;
use bevy_http_client::HttpClient;
use bevy_http_client::prelude::{HttpTypedRequestTrait, TypedRequest, TypedResponse};
use bevy_renet2::netcode::{ClientAuthentication, NETCODE_USER_DATA_BYTES};
use bevy_renet2::prelude::{ConnectionConfig, RenetClient, client_connected, client_disconnected};
use renet2_netcode::{
    NetcodeClientTransport, NetcodeTransportError, ServerCertHash, WebServerDestination,
};
use serde::{Deserialize, Serialize};
use shared::Player;
use shared::event::GameEvent;
use shared::the_hidden_card::prelude::GameState;

#[cfg(target_arch = "wasm32")]
use crate::network::wasm::create_renet_client;
pub(crate) fn plugin(app: &mut App) {
    app.register_request_type::<ClientConnectionInfo>();
    app.add_systems(OnEnter(ScreenState::Title), send_request.run_if(run_once));
    app.add_systems(Update, handle_response.run_if(in_state(ScreenState::Title)));
    app.add_systems(
        PreUpdate,
        try_reconnect
            .run_if(client_disconnected)
            .run_if(resource_exists::<RenetClient>),
    );

    app.add_event::<MessageEvent>()
        .add_observer(send_message_to_server);

    app.add_systems(
        Update,
        send_launch_event
            .run_if(resource_exists::<RenetClient>)
            .run_if(client_connected),
    );
}

#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct ClientConnectionInfo {
    pub native_addr: String,
    pub wt_dest: WebServerDestination,
    pub ws_url: url::Url,
    pub cert_hash: ServerCertHash,
}

fn send_request(mut event_request: EventWriter<TypedRequest<ClientConnectionInfo>>) {
    info!("send request");
    event_request.write(
        HttpClient::new()
            .get("http://127.0.0.1:8080/info")
            .with_type::<ClientConnectionInfo>(),
    );
}

fn handle_response(
    mut cmds: Commands,
    mut events: EventReader<TypedResponse<ClientConnectionInfo>>,
    user: Res<Player>,
) {
    for response in events.read() {
        info!("response received");
        let client_info = response.inner().clone();
        info!("{:?}", client_info);
        cmds.insert_resource(client_info.clone());
        let (client, transport) = create_renet_client(user.deref(), client_info).unwrap();
        cmds.insert_resource(client);
        cmds.insert_resource(transport);
    }
}

fn try_reconnect(
    mut cmds: Commands,
    mut transport_errors: EventWriter<NetcodeTransportError>,
    mut transport: ResMut<NetcodeClientTransport>,
    mut client: ResMut<RenetClient>,
    client_info: Res<ClientConnectionInfo>,
    user: Res<Player>,
) {
    cmds.remove_resource::<RenetClient>();
    cmds.remove_resource::<NetcodeClientTransport>();
    let (client, transport) =
        create_renet_client(user.deref(), client_info.deref().clone()).unwrap();

    cmds.insert_resource(client);
    cmds.insert_resource(transport);
    // if let Err(e) = transport.send_packets(&mut client) {
    //     transport_errors.write(e);
    // }
}

/// 统一处理发送给服务器的游戏事件 [GameEvent]
/// # 试例:
/// ```
/// fn system(mut cmds: Commands) {
///     let event = GameEvent::JoinRoom(O);
///     cmds.write(Message(event));
/// }
/// ```
#[derive(Event)]
pub struct MessageEvent(pub GameEvent);
fn send_message_to_server(
    trigger: Trigger<MessageEvent>,
    mut client: ResMut<RenetClient>,
    bincode_config: Res<BincodeConfig>,
) {
    info!("receive message");
    let message = trigger.event();
    if let Ok(byte) = encode_to_vec(&message.0, bincode_config.0) {
        client.send_message(0, byte);
    } else {
        warn!("message encode error");
    }
}

/// 客户端启动后发送启动事件
fn send_launch_event(mut cmds: Commands, mut sent: Local<bool>, local_user: Res<Player>) {
    if !*sent {
        cmds.trigger(MessageEvent(GameEvent::ClientJustLaunched(local_user.id)));
        *sent = true;
    }
}
