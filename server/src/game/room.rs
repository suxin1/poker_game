use crate::game_server::{RenetGameServer, RenetServerWithConfig};
use bincode::config::Configuration;
use log::{error, info};
use renet2::{ClientId, RenetServer};
use shared::cards::{Card, Deck};
use shared::error::RoomServiceError;
use shared::event::GameEvent;
use shared::the_hidden_card::state::{GameState, Stage};
use shared::{Player, Reducer};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex, RwLock};
use tiny_bail::prelude::r;

type RoomId = u64;

pub struct Room {
    id: RoomId,
    creator_id: ClientId,
    game_state: GameState,
    deck: Deck,
    players: HashSet<ClientId>,
}

impl Room {
    pub fn process_event(&mut self, event: GameEvent, server: &mut RenetServerWithConfig) {
        if !self.game_state.validate(&event) {
            info!("Invalid event: {:?}", event);
            return;
        }
        self.game_state.reduce(&event);
        for client_id in self.players.iter() {
            // TODO 发牌事件处理，不是玩家自己的手牌，考虑隐藏
            server.send_event_next(client_id.clone(), event.clone());
        }

        // 对事件做额外的处理
        match event {
            GameEvent::Ready { client_id: _ } => {
                if self.game_state.is_all_ready() {
                    let event = GameEvent::ToDealCardStage;
                    self.process_event(event, server);

                    // 发牌
                    // TODO 性能优化
                    self.deck.shuffle();
                    let mut deck = VecDeque::from(self.deck.get().clone());
                    let mut hands: HashMap<ClientId, Vec<Card>> = HashMap::new();

                    let seats: Vec<ClientId> = self
                        .game_state
                        .get_seats()
                        .iter()
                        .map(|seat| seat.player.clone().unwrap().id)
                        .collect();

                    for &client_id in &seats {
                        hands.insert(client_id, Vec::with_capacity(13));
                    }

                    for i in 0..52 {
                        if let Some(card) = deck.pop_front() {
                            let seat_index = i % seats.len();
                            let client_id = &seats[seat_index];

                            if let Some(hand) = hands.get_mut(client_id) {
                                hand.push(card);
                            }
                        } else {
                            break;
                        }
                    }

                    // 发送发牌事件给所有玩家
                    // TODO 改为以 seat_index 区分用户，因为手牌是挂在 PlayerSeat上面的
                    for (client_id, hand) in hands.iter_mut() {
                        let event = GameEvent::DealCards {
                            client_id: client_id.clone(),
                            cards: hand.clone(),
                        };
                        self.process_event(event, server);
                    }
                }
            },
            GameEvent::DealCardsDone(client_id) => {
                if self.game_state.is_all_hands_ready() {
                    let caller_index = r!(self.game_state.get_caller_index());
                    let event = GameEvent::ToCallCardStage(caller_index);
                    self.process_event(event, server);
                }
            }
            GameEvent::Pass(_) | GameEvent::PlayCards(_, _) => {
                if let Some(stage) = self.game_state.game_end_check() {
                    if let Stage::Ended(result) = stage {
                        if let Some(result) = result {
                            let event = GameEvent::GameEnd(result);
                            self.process_event(event, server);
                        } else {
                            error!("Game end with error");
                        }
                    }
                }
            }
            _ => {},
        }
    }

    pub fn flush_history(&mut self, client_id: ClientId, server: &mut RenetServerWithConfig) {
        // let history = self.game_state.get_history();
        // history.iter().for_each(|event| {
        //     server.send_event_next(client_id.clone(), event.clone());
        // });

    }

    pub fn sync_state(&mut self, client_id: ClientId, server: &mut RenetServerWithConfig) {
        server.send_event_next(client_id.clone(), GameEvent::SyncState(self.game_state.clone()));
    }

    pub fn join(
        &mut self,
        player: Player,
        server: &mut RenetServerWithConfig,
    ) -> Result<(), RoomServiceError> {
        if !self.game_state.has_empty_seat() {
            return Err(RoomServiceError::RoomFull);
        }

        let seat_index = self.game_state.get_empty_seat_index().unwrap();

        self.add_client(player.id);

        // 发送加入房间成功事件
        server.send_event(player.id, GameEvent::JoinRoomOk { room_id: self.id });
        // 加入房间成功，下一帧将历史事件发送给客户端
        // self.flush_hisotry(player.id.clone(), server);

        self.sync_state(player.id.clone(), server);


        // 下面的所有事件会同步给每一个客户端，每个客户端发送的事件不会直接应用到本地状态，
        // 都会通过服务器验证后才会发送给客户端并应用到本地状态。
        self.process_event(
            GameEvent::AssignSeats {
                player: player.clone(),
                seat_index,
            },
            server,
        );

        Ok(())
    }

    pub fn rejoin(
        &mut self,
        player: Player,
        server: &mut RenetServerWithConfig,
    ) -> Result<(), RoomServiceError> {
        if !self.players.contains(&player.id) {
            return Err(RoomServiceError::ClientNotInRoom);
        }

        // 当前帧发送重新加入房间成功事件
        server.send_event(player.id, GameEvent::ReJoinRoomOk { room_id: self.id });
        // 加入房间成功，下一帧将历史事件发送给客户端
        // self.flush_hisotry(player.id.clone(), server);
        self.sync_state(player.id.clone(), server);

        Ok(())
    }

    pub fn add_client(&mut self, client_id: ClientId) {
        self.players.insert(client_id);
    }

    pub fn remove_client(&mut self, client_id: ClientId) {
        self.players.remove(&client_id);
    }
}

pub struct Rooms {
    rooms: HashMap<RoomId, Arc<RwLock<Room>>>,

    client_room_map: HashMap<ClientId, RoomId>,

    next_room_id: RoomId,
    // TODO 销毁房间, 房间所有用户断开连接后立即销毁房间或60秒后销毁房间或立即销毁
}

impl Rooms {
    pub fn with_test_room() -> Self {
        let mut rooms = Self {
            rooms: HashMap::new(),
            client_room_map: HashMap::new(),
            next_room_id: 0,
        };
        let room_id = rooms.next_room_id;
        let room = Arc::new(RwLock::new(Room {
            id: room_id,
            game_state: GameState::default(),
            players: HashSet::new(),
            creator_id: 0,
            deck: Deck::new(),
        }));

        rooms.next_room_id += 1;

        rooms.rooms.insert(room_id, room);

        rooms
    }
    pub fn create_room(
        &mut self,
        player: Player,
        server: &mut RenetServerWithConfig,
    ) -> Result<(), RoomServiceError> {
        let room_id = self.next_room_id;
        self.next_room_id += 1;

        let room = Arc::new(RwLock::new(Room {
            id: room_id,
            game_state: GameState::default(),
            creator_id: player.id,
            players: HashSet::new(),
            deck: Deck::new(),
        }));

        self.rooms.insert(room_id, room);

        self.join_room(player, room_id, server)?;

        Ok(())
    }

    pub fn join_room(
        &mut self,
        player: Player,
        room_id: RoomId,
        server: &mut RenetServerWithConfig,
    ) -> Result<(), RoomServiceError> {
        if self.client_room_map.contains_key(&player.id) {
            return Err(RoomServiceError::AlreadyInRoom);
        }

        let room = self
            .rooms
            .get(&room_id)
            .ok_or(RoomServiceError::RoomNotFound)?;

        let mut room = room.write().unwrap();

        let result = room.join(player.clone(), server);

        if let Ok(()) = result {
            self.client_room_map.insert(player.id.clone(), room_id);
        }
        result
    }

    pub fn rejoin_room(
        &mut self,
        player: Player,
        server: &mut RenetServerWithConfig,
    ) -> Result<(), RoomServiceError> {
        let Some(room_id) = self.client_room_map.get(&player.id) else {
            return Err(RoomServiceError::ClientNotInRoom);
        };

        let room = self
            .rooms
            .get(&room_id)
            .ok_or(RoomServiceError::RoomNotFound)?;

        let mut room = room.write().unwrap();

        let result = room.rejoin(player.clone(), server);

        result
    }

    /// 方便开发 重置服务器
    fn reset_room(&mut self, room_id: RoomId) -> Result<(), RoomServiceError> {
        let room = self
            .rooms
            .get(&room_id)
            .ok_or(RoomServiceError::RoomNotFound)?;

        let mut room = room.write().unwrap();

        room.game_state = GameState::default();
        self.client_room_map.clear();
        info!("Reset room: {}", room_id);
        Ok(())
    }

    /// 重置服务器
    fn reset_server(&mut self) {
        *self = Self::with_test_room();
    }

    /// 尝试处理事件，如果事件是创建房间或者加入房间，则处理，否则尝试获取房间并将事件交给房间处理
    pub fn process_event(
        &mut self,
        client_id: ClientId,
        event: GameEvent,
        server: &mut RenetServerWithConfig,
    ) -> Result<(), RoomServiceError> {
        match event {
            GameEvent::SyncState(_) | GameEvent::GameEnd(_) => {
                // 阻止非法事件
                Err(RoomServiceError::ActionNotAllowed)
            }
            GameEvent::ServerReset => {
                self.reset_server();
                Ok(())
            }
            GameEvent::ClientJustLaunched(client_id) => {
                let room_id = self.client_room_map.get(&client_id);

                if let Some(room_id) = room_id {
                    let room = self
                        .rooms
                        .get(room_id);
                    if let Some(room) = room {
                        // 向用户发送确认重新加入房间事件
                        server.send_event(client_id, GameEvent::AskForRejoinRoom(room_id.clone()))
                    } else {
                        // 房间已经不存在，将玩家移除 [ClientId] - [RoomId] 映射
                        self.client_room_map.remove(&client_id);
                    }
                }
                Ok(())
            }
            GameEvent::CreateRoom { player } => self.create_room(player, server),
            GameEvent::JoinRoom { player, room_id } => self.join_room(player, room_id, server),
            GameEvent::RoomReset { room_id } => self.reset_room(room_id),
            GameEvent::ReJoinRoom { player} => self.rejoin_room(player, server),
            GameEvent::PlayerConnected(client_id) => {
                let room_id = self.client_room_map.get(&client_id);
                // 如果玩家已经加入房间，则将事件交给房间处理
                if let Some(room_id) = room_id {
                    let room = self
                        .rooms
                        .get(room_id);
                    if let Some(room) = room {
                        room.write().unwrap().process_event(event.clone(), server);
                    } else {
                        // 房间已经不存在，将玩家移除 [ClientId] - [RoomId] 映射
                        self.client_room_map.remove(&client_id);
                    }
                }
                Ok(())
            },
            GameEvent::PlayerLeave(client_id) => {
                let room_id = self.client_room_map.get(&client_id);
                // 如果玩家已经加入房间，则将事件交给房间处理
                if let Some(room_id) = room_id {
                    let room = self
                        .rooms
                        .get(room_id);
                    if let Some(room) = room {
                        room.write().unwrap().process_event(event.clone(), server);
                    }
                    // 房间已经不存在，将玩家移除 [ClientId] - [RoomId] 映射
                    self.client_room_map.remove(&client_id);
                }
                Ok(())
            }
            _ => {
                let room_id = self
                    .client_room_map
                    .get(&client_id)
                    .ok_or(RoomServiceError::ClientNotInRoom)?;

                let room = self
                    .rooms
                    .get(room_id)
                    .ok_or(RoomServiceError::RoomNotFound)?;

                room.write().unwrap().process_event(event.clone(), server);

                Ok(())
            },
        }
    }
}
