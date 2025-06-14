use std::time::{Duration, Instant};
use bincode::config::Configuration;
use log::{info, trace};
use renet2::{ConnectionConfig, RenetServer, ServerEvent};
use renet2_netcode::NetcodeServerTransport;
use state::state::GameState;
use crate::name_from_user_data;

pub fn run_renet_server(mut transport: NetcodeServerTransport) {
    let bincode_config = bincode::config::standard();
    let mut game_state = GameState::default();
    let mut server: RenetServer = RenetServer::new(ConnectionConfig::default());

    let mut last_update = Instant::now();
    loop {
        let now = Instant::now();
        let delta_time = now - last_update;
        last_update = now;

        server.update(delta_time);
        transport.update(delta_time, &mut server);

        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    info!("Client connected {}", client_id);
                    // 通知刚加入的玩家关于其他玩家的信息
                    for (player_id, player) in game_state.players.iter() {
                        let event = state::event::GameEvent::PlayerJoined {
                            player_id: *player_id,
                            name: player.name.clone(),
                        };
                        server.send_message(
                            client_id,
                            0,
                            bincode::serde::encode_to_vec(&event, bincode_config).unwrap(),
                        )
                    }
                    let user_data = transport.user_data(client_id).unwrap();
                    info!("user data: {}", name_from_user_data(&user_data));
                    let event = state::event::GameEvent::PlayerJoined {
                        player_id: client_id,
                        name: name_from_user_data(&user_data),
                    };
                    game_state.dispatch(&event);
                    server.broadcast_message(
                        0,
                        bincode::serde::encode_to_vec(&event, bincode_config).unwrap(),
                    );

                    // 检测是否可以开始游戏
                    if game_state.players.len() == 2 {
                        let event = state::event::GameEvent::BeginGame { first: client_id };
                        game_state.dispatch(&event);

                        // 通知所有玩家开始游戏事件
                        server.broadcast_message(
                            0,
                            bincode::serde::encode_to_vec(&event, bincode_config).unwrap(),
                        );
                        info!("The game gas begun");
                    }
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    let event = state::event::GameEvent::PlayerDisconnected {
                        player_id: client_id,
                    };
                    game_state.dispatch(&event);
                    server.broadcast_message(
                        0,
                        bincode::serde::encode_to_vec(&event, bincode_config).unwrap(),
                    );
                    info!("Client {} disconnected: {}", client_id, reason);

                    let event = state::event::GameEvent::EndGame {
                        reason: state::event::EndGameReason::PlayerLeft {
                            player_id: client_id,
                        },
                    };
                    game_state.dispatch(&event);
                    server.broadcast_message(
                        0,
                        bincode::serde::encode_to_vec(&event, bincode_config).unwrap(),
                    )
                }
            }
        }

        for (client_id) in server.clients_id() {
            while let Some(message) = server.receive_message(client_id, 0) {
                info!("Received event from client {:?}", client_id);
                if let Ok((event, size)) = bincode::serde::decode_from_slice::<
                    state::event::GameEvent,
                    Configuration,
                >(&message, bincode_config)
                {
                    if game_state.validate(&event) {
                        game_state.reduce(&event);
                        trace!("Player {} sent:\n\t{:#?}", client_id, event);
                        server.broadcast_message(
                            0,
                            bincode::serde::encode_to_vec(&event, bincode_config).unwrap(),
                        );

                        // Determine if a player has won the game
                        if let Some(winner) = game_state.determine_game_result() {
                            let event = state::event::GameEvent::EndGame {
                                reason: state::event::EndGameReason::PlayerWon { winner },
                            };
                            server.broadcast_message(
                                0,
                                bincode::serde::encode_to_vec(&event, bincode_config).unwrap(),
                            );
                        }
                    } else {
                        info!("Invalid event received from client {}", client_id);
                    }
                }
            }
        }

        transport.send_packets(&mut server);
        // Control the server running frequency
        std::thread::sleep(Duration::from_millis(50));
    }
}