use std::collections::HashMap;
use std::time::{Duration, Instant};

use bincode::{config::Configuration, serde::decode_from_slice};
use log::{error, info, trace};
use renet2::{ClientId, ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use renet2_netcode::NetcodeServerTransport;

use crate::game::Rooms;
use shared::Player;
use shared::event::GameEvent;

pub struct RenetServerWithConfig {
    config: Configuration,
    server: RenetServer,

    // 使用 HashMap 存储每个客户端待发送的事件列表
    event_buffer: HashMap<ClientId, Vec<GameEvent>>,
}

impl RenetServerWithConfig {
    pub fn send_event(&mut self, client_id: ClientId, event: GameEvent) {
        if !self.server.is_connected(client_id) {
            error!("Client disconnected: {}", client_id);
            error!("Current connected: {}", self.server.connected_clients());
            return;
        }
        info!("Send event: {} to client: {}", event, client_id);
        self.server.send_message(
            client_id.clone(),
            0,
            bincode::serde::encode_to_vec(&event, bincode::config::standard()).unwrap(),
        );
    }

    /// 延迟到下一帧发送事件
    pub fn send_event_next(&mut self, client_id: ClientId, event: GameEvent) {
        self.event_buffer
            .entry(client_id)
            .or_insert_with(Vec::new)
            .push(event);
    }

    /// 刷新所有缓冲事件（在游戏循环结束时调用）
    pub fn flush_events(&mut self) {
        let events_to_send = self.event_buffer.drain().collect::<Vec<_>>();
        for (client_id, events) in events_to_send {
            for event in events {
                self.send_event(client_id, event.clone());
            }
        }
    }
}

pub struct RenetGameServer {
    bincode_config: Configuration,

    server: RenetServerWithConfig,
    transport: NetcodeServerTransport,

    last_update: Instant,

    room_manager: Rooms,
    client_player_cache: HashMap<ClientId, Player>,
}

impl RenetGameServer {
    pub fn with_transport(transport: NetcodeServerTransport) -> Self {
        let bincode_config = bincode::config::standard();
        let server = RenetServer::new(ConnectionConfig {
            available_bytes_per_tick: 60_000,
            server_channels_config: DefaultChannel::config(),
            client_channels_config: DefaultChannel::config(),
        });
        Self {
            bincode_config,
            server: RenetServerWithConfig {
                config: bincode_config,
                server,
                event_buffer: HashMap::new(),
            },
            last_update: Instant::now(),
            transport,
            room_manager: Rooms::with_test_room(), // 测试房间id为u64默认值 0
            client_player_cache: HashMap::new(),
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let delta_time = now - self.last_update;
        self.last_update = now;

        self.server.server.update(delta_time);
        self.transport.update(delta_time, &mut self.server.server);

        while let Some(event) = self.server.server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    // 处理用户重新连接，恢复用户状态
                    info!("Client connected: {}", client_id);
                    let _ = self.room_manager.process_event(client_id, GameEvent::PlayerConnected(client_id), &mut self.server);
                },
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    // 处理用户断开连接， 更新用户状态为离线
                    info!("Client disconnected: {}", client_id);
                    let _ = self.room_manager.process_event(client_id, GameEvent::PlayerDisconnected(client_id), &mut self.server);
                },
            }
        }

        // 清空并发送上一帧需缓存的事件
        self.server.flush_events();

        for (client_id) in self.server.server.clients_id() {
            while let Some(message) = self.server.server.receive_message(client_id, 0) {
                if let Ok((event, _)) =
                    decode_from_slice::<GameEvent, Configuration>(&message, self.bincode_config)
                {
                    info!("Received event from client {:?}, {}", client_id, event);
                    let res =
                        self.room_manager
                            .process_event(client_id, event.clone(), &mut self.server);
                    if let Err(err) = res {
                        info!(
                            "Error processing event from client {:?}, {}",
                            client_id, err
                        );
                    }
                }
            }
        }

        self.transport.send_packets(&mut self.server.server);
        std::thread::sleep(Duration::from_millis(50));
    }
}
