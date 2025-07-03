use crate::prelude::*;
use std::ops::Deref;
use web_time::SystemTime;

use bevy_http_client::HttpClient;
use bevy_http_client::prelude::{HttpTypedRequestTrait, TypedRequest, TypedResponse};
use bevy_renet2::netcode::{ClientAuthentication, NETCODE_USER_DATA_BYTES};
use bevy_renet2::prelude::{ConnectionConfig, RenetClient, client_disconnected, ChannelConfig, DefaultChannel};
use renet2_netcode::{
    NetcodeClientTransport, NetcodeTransportError, ServerCertHash, WebServerDestination,
};
use serde::{Deserialize, Serialize};

use crate::network::{PROTOCOL_ID, SERVER_ADDR};
use renet2_netcode::{
    ClientSocket, WebSocketClient, WebSocketClientConfig, WebTransportClient,
    WebTransportClientConfig, webtransport_is_available_with_cert_hashes,
};
use shared::Player;


use crate::network::init::ClientConnectionInfo;

pub(super) fn create_renet_client(
    user: &Player,
    client_connection_info: ClientConnectionInfo,
) -> anyhow::Result<(RenetClient, NetcodeClientTransport)> {
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;

    info!("Try connect");
    let username = user.name.clone();
    let client_id = user.id.clone();

    let mut user_data = [0u8; NETCODE_USER_DATA_BYTES];
    if username.len() > NETCODE_USER_DATA_BYTES - 8 {
        panic!("Username is too long");
    }
    user_data[0..8].copy_from_slice(&(username.len() as u64).to_le_bytes());
    user_data[8..username.len() + 8].copy_from_slice(username.as_bytes());

    let (client, transport) = {
        let mut url = client_connection_info.ws_url.clone();
        let _ = url.set_host(Some(SERVER_ADDR));

        let socket_config = WebSocketClientConfig {
            server_url: url,
        };

        let socket = WebSocketClient::new(socket_config).unwrap();
        let client = RenetClient::new(ConnectionConfig {
            available_bytes_per_tick: 60_000,
            server_channels_config: DefaultChannel::config(),
            client_channels_config: DefaultChannel::config(),
        }, socket.is_reliable());

        let client_auth = ClientAuthentication::Unsecure {
            client_id: client_id as u64,
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

