use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use log::info;
use crate::event::{EndGameReason, GameEvent};

pub(crate) type PlayerId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Tile {
    Empty,
    Tic,
    Tac,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub piece: Tile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stage {
    PreGame,
    InGame,
    Ended
}

pub struct GameState {
    pub stage: Stage,
    pub board: [Tile; 9],
    pub active_player_id: PlayerId,
    pub players: HashMap<PlayerId, Player>,
    pub history: Vec<GameEvent>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            stage: Stage::PreGame,
            board: [Tile::Empty; 9],
            active_player_id: 0,
            players: HashMap::new(),
            history: Vec::new(),
        }
    }
}

impl GameState {
    pub fn validate(&self, event: &GameEvent) -> bool {
        use GameEvent::*;
        match event {
            BeginGame {first} => {
                println!("BeginGame");
                // 检查初始玩家是否存在
                if !self.players.contains_key(first) {
                    return false;
                }
                // 检查游戏状态, 处于预游戏状态才能启动游戏
                if self.stage != Stage::PreGame {
                    return false;
                }
            }
            EndGame {reason} => match reason {
                EndGameReason::PlayerWon {winner: _} => {
                    if self.stage != Stage::InGame {
                        return false;
                    }
                }
                _ => {}
            }
            PlayerJoined { player_id, name: _} => {
                if self.players.contains_key(player_id) {
                    return false;
                }
            }
            PlayerDisconnected {player_id} => {
                if !self.players.contains_key(player_id) {
                    return false;
                }
            }
            PlaceTile { player_id, at } => {
                // Check player exists
                println!("Player {} try to place at {}", player_id, at);
                if !self.players.contains_key(player_id) {
                    return false;
                }
                // Check player is currently the one making their move
                if self.active_player_id != *player_id {
                    println!("Player {} try to place at {} rejected", player_id, at);
                    return false;
                }

                // Check that the tile index is inside the board
                if *at > 8 {
                    return false;
                }

                // Check that the player is not trying to place piece on top of another
                // (which is considered a cheeky move in tic tac toe)
                if self.board[*at] != Tile::Empty {
                    return false;
                }
            }
        }

        true
    }
}

impl GameState {
    // validate ...

    /// Consumes an event, modifying the GameState and adding the event to its history
    /// NOTE: consume assumes the event to have already been validated and will accept *any* event passed to it
    pub fn reduce(&mut self, valid_event: &GameEvent) {
        use GameEvent::*;
        match valid_event {
            BeginGame { first } => {
                self.active_player_id = *first;
                self.stage = Stage::InGame;
            }
            EndGame { reason: _ } => self.stage = Stage::Ended,
            PlayerJoined { player_id, name } => {
                self.players.insert(
                    *player_id,
                    Player {
                        name: name.to_string(),
                        // First player to join gets tac, second gets tic
                        piece: if self.players.len() > 0 {
                            Tile::Tac
                        } else {
                            Tile::Tic
                        },
                    },
                );
            }
            PlayerDisconnected { player_id } => {
                self.players.remove(player_id);
            }
            PlaceTile { player_id, at } => {
                let piece = self.players.get(player_id).unwrap().piece;
                self.board[*at] = piece;
                self.active_player_id = self
                    .players
                    .keys()
                    .find(|id| *id != player_id)
                    .unwrap()
                    .clone();
            }
        }

        self.history.push(valid_event.clone());
    }

    pub fn determine_game_result(&self) -> Option<PlayerId> {
        // All the combinations of 3 tiles that wins the game
        let row1: [usize; 3] = [0, 1, 2];
        let row2: [usize; 3] = [3, 4, 5];
        let row3: [usize; 3] = [6, 7, 8];
        let col1: [usize; 3] = [0, 3, 6];
        let col2: [usize; 3] = [1, 4, 7];
        let col3: [usize; 3] = [2, 5, 8];
        let diag1: [usize; 3] = [0, 4, 8];
        let diag2: [usize; 3] = [2, 4, 6];

        for arr in [row1, row2, row3, col1, col2, col3, diag1, diag2] {
            // Read tiles from board
            let tiles: [Tile; 3] = [self.board[arr[0]], self.board[arr[1]], self.board[arr[2]]];
            // Determine if tiles are all equal
            let all_are_the_same = tiles
                .get(0)
                .map(|first| tiles.iter().all(|x| x == first))
                .unwrap_or(true);

            if all_are_the_same {
                // Determine which of the players won
                if let Some((winner, _)) = self
                    .players
                    .iter()
                    .find(|(_, player)| player.piece == self.board[arr[0]])
                {
                    return Some(*winner);
                }
            }
        }

        None
    }

    pub fn dispatch(&mut self, event: &GameEvent) -> Result<(), ()> {
        if !self.validate(event) {
            return Err(());
        }
        self.reduce(event);
        Ok(())
    }

    pub fn get_player_piece(&self, player_id: &PlayerId) -> Option<Tile> {
        if let Some(player) = self.players.get(player_id) {
           return Some(player.piece);
        }
        None
    }
}
