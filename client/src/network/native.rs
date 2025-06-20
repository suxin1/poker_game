use bevy::prelude::*;
use std::net::{SocketAddr, UdpSocket};
use std::time::SystemTime;

use crate::network::PROTOCOL_ID;
use crate::network::init::ClientConnectionInfo;
use crate::screens::ScreenState;

use bevy_http_client::prelude::{HttpTypedRequestTrait, TypedRequest, TypedResponse};
use bevy_renet2::netcode::{ClientAuthentication, NETCODE_USER_DATA_BYTES};
use bevy_renet2::prelude::{ConnectionConfig, DefaultChannel, RenetClient};
use renet2_netcode::{NativeSocket, NetcodeClientTransport, ServerCertHash, WebServerDestination};

use serde::{Deserialize, Serialize};
use shared::Player;

// Create a RenetClient that already connected to a server.
// Returns an Err if connection fails
pub(super) fn create_renet_client(
    user: &Player,
    client_connection_info: ClientConnectionInfo,
) -> anyhow::Result<(RenetClient, NetcodeClientTransport)> {
    let server_addr: SocketAddr = client_connection_info.native_addr.parse()?;
    let socket = NativeSocket::new(UdpSocket::bind("127.0.0.1:0")?).unwrap();

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
