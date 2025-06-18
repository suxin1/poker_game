mod http_server;
mod game_server;
mod game;
mod utils;

use std::env;
use std::{
    net::{SocketAddr, UdpSocket, IpAddr},
    time::{Duration, Instant, SystemTime},
    str::FromStr
};

use bincode::config::Configuration;

use log::{info, trace};

use renet2::{ConnectionConfig, RenetServer, ServerEvent};
use renet2_netcode::{
    BoxedSocket, NETCODE_USER_DATA_BYTES, NativeSocket, NetcodeServerTransport,
    ServerAuthentication, ServerCertHash, ServerConfig, ServerSetupConfig, WebServerDestination,
    WebSocketServer, WebSocketServerConfig, WebTransportServer, WebTransportServerConfig,
    ServerSocket
};
use serde::{Deserialize, Serialize};
use crate::game_server::RenetGameServer;
use crate::http_server::run_http_server;

// used to make sure players use the most recent version of the client.
pub const PROTOCOL_ID: u64 = 7;

const SERVER_ADDR: &str = "127.0.0.1:5000";

/// Utility function for extracting a players name from renet user data
fn name_from_user_data(user_data: &[u8; NETCODE_USER_DATA_BYTES]) -> String {
    let mut buffer = [0u8; 8];
    buffer.copy_from_slice(&user_data[0..8]);
    let mut len = u64::from_le_bytes(buffer) as usize;
    len = len.min(NETCODE_USER_DATA_BYTES - 8);
    let data = user_data[8..len + 8].to_vec();
    String::from_utf8(data).unwrap()
}


#[derive(Serialize, Deserialize, Clone, Debug)]
struct ClientConnectionInfo {
    native_addr: String,
    wt_dest: WebServerDestination,
    ws_url: url::Url,
    cert_hash: ServerCertHash,
}

fn main() {
    env_logger::init();
    info!("Starting server");

    let runtime = tokio::runtime::Runtime::new().unwrap();

    let max_clients = env::var("MAX_CLIENT").unwrap().parse::<usize>().unwrap();
    let http_addr = env::var("HTTP_SERVER_ADDR").unwrap().parse::<SocketAddr>().unwrap();

    let native_socket_addr: SocketAddr = "127.0.0.1:8081".parse().unwrap();
    let wt_socket_addr: SocketAddr = "127.0.0.1:8082".parse().unwrap();
    let ws_socket_addr: SocketAddr = "127.0.0.1:8083".parse().unwrap();

    // let native_socket_addr: SocketAddr = "127.0.0.1:8081".parse().unwrap();
    // let wt_socket_addr: SocketAddr = "127.0.0.1:8082".parse().unwrap();
    // let ws_socket_addr: SocketAddr = "127.0.0.1:8083".parse().unwrap();

    // Native socket
    let native_socket = NativeSocket::new(UdpSocket::bind(native_socket_addr).unwrap()).unwrap();

    // WebTransport socket
    let (wt_socket, cert_hash) = {
        let (config, cert_hash) =
            WebTransportServerConfig::new_selfsigned(wt_socket_addr, max_clients).unwrap();
        (
            WebTransportServer::new(config, runtime.handle().clone()).unwrap(),
            cert_hash,
        )
    };

    // WebSocket socket
    let ws_socket = {
        let config = WebSocketServerConfig::new(ws_socket_addr, max_clients);
        WebSocketServer::new(config, runtime.handle().clone()).unwrap()
    };

    let client_connection_info = ClientConnectionInfo {
        native_addr: native_socket.addr().unwrap().to_string(),
        wt_dest: wt_socket.addr().unwrap().into(),
        ws_url: ws_socket.url(),
        cert_hash,
    };

    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let server_config = ServerSetupConfig {
        current_time,
        max_clients: max_clients,
        protocol_id: PROTOCOL_ID,
        socket_addresses: vec![
            vec![native_socket.addr().unwrap()],
            vec![wt_socket.addr().unwrap()],
            vec![ws_socket.addr().unwrap()],
        ],
        authentication: ServerAuthentication::Unsecure,
    };

    // let mut transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    let transport = NetcodeServerTransport::new_with_sockets(
        server_config,
        Vec::from([
            BoxedSocket::new(native_socket),
            BoxedSocket::new(wt_socket),
            BoxedSocket::new(ws_socket),
        ]),
    )
    .unwrap();
    let mut renet_game_server = RenetGameServer::with_transport(transport);
    runtime.spawn(async move { run_http_server(http_addr, client_connection_info).await});

    loop {
        renet_game_server.update();
    }
}
