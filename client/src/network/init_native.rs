use std::net::{SocketAddr, UdpSocket};
use std::time::SystemTime;
use bevy::prelude::*;

use bevy_http_client::HttpClient;
use bevy_http_client::prelude::{HttpTypedRequestTrait, TypedRequest, TypedResponse};
use bevy_renet2::netcode::{ClientAuthentication, NETCODE_USER_DATA_BYTES};
use bevy_renet2::prelude::{ConnectionConfig, DefaultChannel, RenetClient};
use renet2_netcode::{NetcodeClientTransport, ServerCertHash, WebServerDestination};
use serde::{Deserialize, Serialize};
use crate::screens::ScreenState;

pub(crate) fn plugin(app: &mut App) {

        let (client, transport) = create_renet_client(&"suxin".to_string()).unwrap();
        app.insert_resource(client);
        app.insert_resource(transport);

}

const PROTOCOL_ID: u64 = 7;

const SERVER_ADDR: &str = "127.0.0.1:8080";

// Create a RenetClient that already connected to a server.
// Returns an Err if connection fails
pub(super) fn create_renet_client(username: &String) -> anyhow::Result<(RenetClient, NetcodeClientTransport)> {
    let server_addr: SocketAddr = SERVER_ADDR.parse()?;

    let socket = bevy_renet2::netcode::NativeSocket::new(UdpSocket::bind("127.0.0.1:0")?).unwrap();

    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let client_id = current_time.as_millis() as u64;

    info!("Try connect");
    let mut user_data = [0u8; NETCODE_USER_DATA_BYTES];
    if username.len() > NETCODE_USER_DATA_BYTES - 8 {
        panic!("Username is too long");
    }
    user_data[0..8].copy_from_slice(&(username.len() as u64).to_le_bytes());
    user_data[8..username.len() + 8].copy_from_slice(username.as_bytes());

    let authentication = ClientAuthentication::Unsecure {
        server_addr,
        client_id,
        socket_id: 0,
        user_data: Some(user_data),
        protocol_id: PROTOCOL_ID,
    };
    let mut transport = NetcodeClientTransport::new(current_time, authentication, socket)?;
    let client = RenetClient::new(
        ConnectionConfig {
            // At 60hz this is becomes 28.8 Mbps
            available_bytes_per_tick: 60_000,
            server_channels_config: DefaultChannel::config(),
            client_channels_config: DefaultChannel::config(),
        },
        false,
    );

    Ok((client, transport))
}