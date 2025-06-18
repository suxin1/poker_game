use std::time::{Duration, Instant};
use std::collections::HashMap;

use bincode::{config::Configuration, serde::decode_from_slice};
use log::{info, trace};
use renet2::{ClientId, ConnectionConfig, RenetServer, ServerEvent};
use renet2_netcode::NetcodeServerTransport;


use crate::name_from_user_data;
use crate::game::Rooms;
use shared::event::GameEvent;
use shared::Player;
use shared::the_hidden_card::prelude::*;

pub struct RenetGameServer {
    bincode_config: Configuration,

    server: RenetServer,
    transport: NetcodeServerTransport,

    last_update: Instant,

    room_manager: Rooms,
    client_player_cache: HashMap<ClientId, Player>,
}

impl RenetGameServer {
    pub fn with_transport(transport: NetcodeServerTransport) -> Self {
        let bincode_config = bincode::config::standard();
        let server = RenetServer::new(ConnectionConfig::default());

        Self {
            bincode_config,
            server,
            last_update: Instant::now(),
            transport,
            room_manager: Rooms::with_test_room(),  // 测试房间id为u64默认值 0
            client_player_cache: HashMap::new(),
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let delta_time = now - self.last_update;
        self.last_update = now;

        self.server.update(delta_time);
        self.transport.update(delta_time, &mut self.server);

        while let Some(event) = self.server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    // 处理用户重新连接，恢复用户状态
                    info!("Client connected: {}", client_id);
                },
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    // 处理用户断开连接， 更新用户状态为离线
                    info!("Client disconnected: {}", client_id);
                },
            }
        }

        for (client_id) in self.server.clients_id() {
            while let Some(message) = self.server.receive_message(client_id, 0) {
                info!("Received event from client {:?}", client_id);
                if let Ok((event, _)) =
                    decode_from_slice::<GameEvent, Configuration>(&message, self.bincode_config)
                {
                    self.room_manager.process_event(client_id, event.clone(), &mut self.server);
                }
            }
        }

        self.transport.send_packets(&mut self.server);
        std::thread::sleep(Duration::from_millis(50));
    }
}
