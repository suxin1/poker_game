use std::ops::Deref;
use web_time::SystemTime;
use crate::prelude::*;

use bevy_http_client::HttpClient;
use bevy_http_client::prelude::{HttpTypedRequestTrait, TypedRequest, TypedResponse};
use bevy_renet2::netcode::{ClientAuthentication, NETCODE_USER_DATA_BYTES};
use bevy_renet2::prelude::{client_disconnected, ConnectionConfig, RenetClient};
use renet2_netcode::{NetcodeClientTransport, NetcodeTransportError, ServerCertHash, WebServerDestination};
use serde::{Deserialize, Serialize};

use renet2_netcode::{
    ClientSocket, WebSocketClient, WebSocketClientConfig, WebTransportClient,
    WebTransportClientConfig, webtransport_is_available_with_cert_hashes,
};
use crate::network::PROTOCOL_ID;
use crate::screens::Screen;

pub(crate) fn plugin(app: &mut App) {
    app.register_request_type::<ClientConnectionInfo>();
    app.add_systems(
        OnEnter(Screen::Title),
        send_request.run_if(run_once),
    );
    app.add_systems(
        Update,
        handle_response.run_if(in_state(Screen::Title)),
    );
    app.add_systems(
        PreUpdate,
        try_reconnect
            .run_if(client_disconnected)
            .run_if(resource_exists::<RenetClient>),
    );
}

#[derive(Resource, Serialize, Deserialize, Clone, Debug)]
pub struct ClientConnectionInfo {
    native_addr: String,
    wt_dest: WebServerDestination,
    ws_url: url::Url,
    cert_hash: ServerCertHash,
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
) {
    for response in events.read() {
        info!("response received");
        let a = response.deref();
        let client_info = response.inner().clone();
        info!("{:?}", client_info);
        cmds.insert_resource(client_info.clone());
        let (client, transport) = create_renet_client(&"suxin".to_string(), client_info).unwrap();
        cmds.insert_resource(client);
        cmds.insert_resource(transport);
    }
}

pub(super) fn create_renet_client(
    username: &String,
    client_connection_info: ClientConnectionInfo,
) -> anyhow::Result<(RenetClient, NetcodeClientTransport)> {
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let client_id = current_time.as_millis() as u64;

    info!("Try connect");
    let mut user_data = [0u8; NETCODE_USER_DATA_BYTES];
    if username.len() > NETCODE_USER_DATA_BYTES - 8 {
        panic!("Username is too long");
    }
    user_data[0..8].copy_from_slice(&(username.len() as u64).to_le_bytes());
    user_data[8..username.len() + 8].copy_from_slice(username.as_bytes());

    let (client, transport) = {
            let socket_config = WebSocketClientConfig {
                server_url: client_connection_info.ws_url.clone(),
            };

            let socket = WebSocketClient::new(socket_config).unwrap();
            let client = RenetClient::new(ConnectionConfig::default(), socket.is_reliable());

            let client_auth = ClientAuthentication::Unsecure {
                client_id: current_time.as_millis() as u64,
                protocol_id: PROTOCOL_ID,
                socket_id: 2, //WebSocket socket id is 2 in this example
                server_addr: socket.server_address(),
                user_data: Some(user_data),
            };
            let transport = NetcodeClientTransport::new(current_time, client_auth, socket).unwrap();

            (client, transport)
    };

    Ok((client, transport))
}

fn try_reconnect(
    mut cmds: Commands,
    mut transport: ResMut<NetcodeClientTransport>,
    mut client: ResMut<RenetClient>,
    client_info: Res<ClientConnectionInfo>,
    mut transport_errors: EventWriter<NetcodeTransportError>,
) {
    cmds.remove_resource::<RenetClient>();
    cmds.remove_resource::<NetcodeClientTransport>();
    let (client, transport)  = create_renet_client(&"suxin".to_string(), client_info.deref().clone()).unwrap();

    cmds.insert_resource(client);
    cmds.insert_resource(transport);
    // if let Err(e) = transport.send_packets(&mut client) {
    //     transport_errors.write(e);
    // }
}
