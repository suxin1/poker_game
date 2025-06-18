use renet2::{ClientId, RenetServer};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

use shared::{Player, Reducer};
use shared::error::RoomServiceError;
use shared::event::GameEvent;
use shared::the_hidden_card::state::GameState;

type RoomId = u64;

pub struct Room {
    id: RoomId,
    creator_id: ClientId,
    game_state: GameState,
    players: HashSet<ClientId>,
}

impl Room {
    pub fn process_event(&mut self, event: GameEvent, server: &mut RenetServer) {
        if !self.game_state.validate(&event) {
            return;
        }
        self.game_state.reducer(&event);
        // server.broadcast_message()
        for client_id in self.players.iter() {
            server.send_message(
                client_id.clone(),
                0,
                bincode::serde::encode_to_vec(&event, bincode::config::standard()).unwrap(),
            );
        }
    }

    pub fn add_client(&mut self, client_id: ClientId) {
        self.players.insert(client_id);
    }

    pub fn remove_client(&mut self, client_id: ClientId) {
        self.players.remove(&client_id);
    }
}

#[derive(Default)]
pub struct Rooms {
    rooms: HashMap<RoomId, Arc<RwLock<Room>>>,

    client_room_map: HashMap<ClientId, RoomId>,

    next_room_id: RoomId,
}

impl Rooms {

    pub fn with_test_room() -> Self {
        let mut rooms = Self::default();
        let room_id = rooms.next_room_id;
        let room = Arc::new(RwLock::new(Room {
            id: room_id,
            game_state: GameState::default(),
            players: HashSet::new(),
            creator_id: 0,
        }));

        rooms.next_room_id += 1;

        rooms.rooms.insert(room_id, room);

        rooms
    }
    pub fn create_room(&mut self, player: Player, server: &mut RenetServer) -> Result<(), RoomServiceError> {
        let room_id = self.next_room_id;
        self.next_room_id += 1;

        let room = Arc::new(RwLock::new(Room {
            id: room_id,
            game_state: GameState::default(),
            creator_id: player.id,
            players: HashSet::new(),
        }));

        self.rooms.insert(room_id, room);

        self.join_room(player, room_id, server)?;

        Ok(())
    }

    pub fn join_room(
        &mut self,
        player: Player,
        room_id: RoomId,
        server: &mut RenetServer,
    ) -> Result<(), RoomServiceError> {
        if self.client_room_map.contains_key(&player.id) {
            return Err(RoomServiceError::AlreadyInRoom);
        }

        let room = self
            .rooms
            .get(&room_id)
            .ok_or(RoomServiceError::RoomNotFound)?;

        let mut room = room.write().unwrap();

        if !room.game_state.has_empty_seat() {
            return Err(RoomServiceError::RoomFull);
        }

        let seat_index = room.game_state.get_empty_seat_index().unwrap();
        room.process_event(GameEvent::AssignSeats { player, seat_index }, server);

        Ok(())
    }

    pub fn process_event(
        &mut self,
        client_id: ClientId,
        event: GameEvent,
        server: &mut RenetServer,
    ) -> Result<(), RoomServiceError> {
        match event {
            GameEvent::CreateRoom { player } => {
                self.create_room(player, server)
            },
            GameEvent::JoinRoom { player, room_id } => {
                self.join_room(player, room_id, server)
            },
            _ => Ok(()),
        }
    }
}
